use anyhow::{Context, Result, bail};
use athenisa_isa::instruction::{Instruction, Register};

const INSTR_MEM_SIZE: i32 = 2048;
const DATA_MEM_SIZE: i32 = 65536;

#[derive(Clone, Copy, PartialEq)]
enum Section {
    Code,
    Data,
}

#[derive(Clone, Copy, PartialEq)]
enum SymbolKind {
    Label,
    Constant,
    Data,
}

pub struct Symbol {
    pub name: String,
    pub value: i32,
    kind: SymbolKind,
}

pub type Symbols = Vec<Symbol>;

struct SourceLine {
    number: usize,
    text: String,
}

struct DataDeclaration {
    name: String,
    size: usize,
    values: String,
}

pub struct ParsedProgram {
    pub instructions: Vec<Instruction>,
    pub data: Vec<u16>,
    pub symbols: Symbols,
}

pub fn parse_source(source: &str) -> Result<ParsedProgram> {
    let lines = prepare_lines(source)?;
    let symbols = collect_symbols(&lines)?;
    let (instructions, data) = parse_program(&lines, &symbols)?;

    Ok(ParsedProgram {
        instructions,
        data,
        symbols,
    })
}

fn prepare_lines(source: &str) -> Result<Vec<SourceLine>> {
    let mut lines = Vec::new();
    let mut section = Section::Code;
    let mut continued_line: Option<SourceLine> = None;

    for (line_idx, raw_line) in source.lines().enumerate() {
        let number = line_idx + 1;
        let text = strip_comment(raw_line).trim();

        if text.is_empty() {
            continue;
        }

        if let Some(mut current_line) = continued_line.take() {
            let next_section =
                parse_section(text).with_context(|| format!("line {number}: {text}"))?;

            if next_section.is_some() {
                bail!(
                    "line {number}: section directive cannot continue a data declaration from line {}",
                    current_line.number
                );
            }

            current_line.text.push(' ');
            current_line.text.push_str(text);

            if current_line.text.ends_with(',') {
                continued_line = Some(current_line);
            } else {
                lines.push(current_line);
            }

            continue;
        }

        let new_section = parse_section(text).with_context(|| format!("line {number}: {text}"))?;

        if let Some(new_section) = new_section {
            section = new_section;
            lines.push(SourceLine {
                number,
                text: text.to_string(),
            });
            continue;
        }

        let source_line = SourceLine {
            number,
            text: text.to_string(),
        };

        if section == Section::Data && text.ends_with(',') {
            continued_line = Some(source_line);
        } else {
            lines.push(source_line);
        }
    }

    if let Some(line) = continued_line {
        bail!(
            "line {}: data initializer cannot end with a comma",
            line.number
        );
    }

    Ok(lines)
}

fn collect_symbols(lines: &[SourceLine]) -> Result<Symbols> {
    let mut symbols = Vec::new();
    let mut section = Section::Code;
    let mut code_pc = 0;
    let mut data_pc = 0;

    for source_line in lines {
        let line_num = source_line.number;
        let line = source_line.text.as_str();

        if let Some(new_section) = parse_section(line)? {
            section = new_section;
            continue;
        }

        if section == Section::Code {
            if is_symbol_line(line) {
                let symbol = parse_symbol(line, code_pc, &symbols)
                    .with_context(|| format!("line {line_num}: {line}"))?;

                define_symbol(&mut symbols, symbol, line_num)?;
                continue;
            }

            if is_data_declaration(line) {
                bail!("line {line_num}: data declarations are only allowed in .data");
            }

            let size = instruction_size(line) as i32;

            if code_pc + size > INSTR_MEM_SIZE {
                bail!(
                    "line {line_num}: program exceeds instruction memory size of {INSTR_MEM_SIZE} words"
                );
            }

            code_pc += size;
        } else {
            if is_symbol_line(line) {
                bail!("line {line_num}: labels and constants are only allowed in .code");
            }

            let declaration = parse_data_declaration(line, &symbols)
                .with_context(|| format!("line {line_num}: {line}"))?;
            let size = declaration.size as i32;

            if size > DATA_MEM_SIZE - data_pc {
                bail!("line {line_num}: data exceeds data memory size of {DATA_MEM_SIZE} words");
            }

            let symbol = Symbol {
                name: declaration.name,
                value: data_pc,
                kind: SymbolKind::Data,
            };

            define_symbol(&mut symbols, symbol, line_num)?;
            data_pc += size;
        }
    }

    Ok(symbols)
}

fn parse_program(lines: &[SourceLine], symbols: &Symbols) -> Result<(Vec<Instruction>, Vec<u16>)> {
    let mut instructions = Vec::new();
    let mut data = Vec::new();
    let mut section = Section::Code;
    let mut code_pc = 0;

    for source_line in lines {
        let line_num = source_line.number;
        let line = source_line.text.as_str();

        if let Some(new_section) = parse_section(line)? {
            section = new_section;
            continue;
        }

        if section == Section::Code {
            if is_symbol_line(line) {
                continue;
            }

            let new_instructions = parse_instruction(line, code_pc, symbols)
                .with_context(|| format!("line {line_num}: {line}"))?;
            code_pc += new_instructions.len() as i32;
            instructions.extend(new_instructions);
        } else {
            let declaration = parse_data_declaration(line, symbols)
                .with_context(|| format!("line {line_num}: {line}"))?;
            let words = parse_data_values(&declaration, symbols)
                .with_context(|| format!("line {line_num}: {line}"))?;
            data.extend(words);
        }
    }

    Ok((instructions, data))
}

fn parse_section(line: &str) -> Result<Option<Section>> {
    if line.eq_ignore_ascii_case(".code") {
        return Ok(Some(Section::Code));
    }

    if line.eq_ignore_ascii_case(".data") {
        return Ok(Some(Section::Data));
    }

    if line.starts_with('.') {
        bail!("unknown directive '{line}'");
    }

    Ok(None)
}

fn is_symbol_line(line: &str) -> bool {
    line.contains(':')
}

fn is_data_declaration(line: &str) -> bool {
    let Some(open_bracket) = line.find('[') else {
        return false;
    };

    let name = &line[..open_bracket];

    is_symbol_name(name)
}

fn strip_comment(line: &str) -> &str {
    line.split(';').next().unwrap()
}

fn parse_symbol(line: &str, pc: i32, symbols: &Symbols) -> Result<Symbol> {
    let colon_idx = line.find(':').unwrap();
    let name = line[..colon_idx].trim();
    let value_text = line[colon_idx + 1..].trim();

    if name.is_empty() {
        bail!("symbol name cannot be empty");
    }

    if !is_symbol_name(name) {
        bail!("invalid symbol name '{}'", name);
    }

    let kind = if value_text.is_empty() {
        SymbolKind::Label
    } else {
        SymbolKind::Constant
    };
    let value = parse_symbol_value(value_text, pc, symbols)?;

    Ok(Symbol {
        name: name.to_string(),
        value,
        kind,
    })
}

fn define_symbol(symbols: &mut Symbols, symbol: Symbol, line_num: usize) -> Result<()> {
    if symbols.iter().any(|existing| existing.name == symbol.name) {
        bail!(
            "line {line_num}: symbol '{}' is already defined",
            symbol.name
        );
    }

    symbols.push(symbol);

    Ok(())
}

fn is_symbol_name(name: &str) -> bool {
    let mut chars = name.chars();

    let Some(first_char) = chars.next() else {
        return false;
    };

    if !first_char.is_ascii_alphabetic() && first_char != '_' {
        return false;
    }

    for current_char in chars {
        if !current_char.is_ascii_alphanumeric() && current_char != '_' {
            return false;
        }
    }

    true
}

fn parse_symbol_value(text: &str, pc: i32, symbols: &Symbols) -> Result<i32> {
    if text.is_empty() {
        return Ok(pc);
    }

    parse_expression(text, symbols)
}

fn parse_expression(text: &str, symbols: &Symbols) -> Result<i32> {
    parse_expression_inner(text, symbols, false)
}

fn parse_size_expression(text: &str, symbols: &Symbols) -> Result<i32> {
    parse_expression_inner(text, symbols, true)
}

fn parse_expression_inner(text: &str, symbols: &Symbols, constants_only: bool) -> Result<i32> {
    let text = text.trim();

    if text.is_empty() {
        bail!("expected a value");
    }

    if let Some((operator_idx, operator)) = find_operator(text, &['+', '-'])? {
        let left = parse_expression_inner(&text[..operator_idx], symbols, constants_only)?;
        let right = parse_expression_inner(&text[operator_idx + 1..], symbols, constants_only)?;

        return calculate_expression(left, operator, right);
    }

    if let Some((operator_idx, operator)) = find_operator(text, &['*', '/', '%'])? {
        let left = parse_expression_inner(&text[..operator_idx], symbols, constants_only)?;
        let right = parse_expression_inner(&text[operator_idx + 1..], symbols, constants_only)?;

        return calculate_expression(left, operator, right);
    }

    if has_outer_parentheses(text) {
        return parse_expression_inner(&text[1..text.len() - 1], symbols, constants_only);
    }

    if let Ok(value) = parse_number(text) {
        return Ok(value);
    }

    if let Some(value_text) = text.strip_prefix('+') {
        return parse_expression_inner(value_text, symbols, constants_only);
    }

    if let Some(value_text) = text.strip_prefix('-') {
        let value = parse_expression_inner(value_text, symbols, constants_only)?;

        return value
            .checked_neg()
            .context("expression result is out of i32 range");
    }

    if constants_only {
        parse_constant_value(text, symbols)
    } else {
        parse_value(text, symbols)
    }
}

fn find_operator(text: &str, operators: &[char]) -> Result<Option<(usize, char)>> {
    let mut parentheses = 0;
    let mut found_operator = None;

    for (index, character) in text.char_indices() {
        if character == '(' {
            parentheses += 1;
            continue;
        }

        if character == ')' {
            if parentheses == 0 {
                bail!("unexpected ')' in expression");
            }

            parentheses -= 1;
            continue;
        }

        if parentheses == 0 && operators.contains(&character) && is_binary_operator(text, index) {
            found_operator = Some((index, character));
        }
    }

    if parentheses != 0 {
        bail!("expected ')' in expression");
    }

    Ok(found_operator)
}

fn is_binary_operator(text: &str, operator_idx: usize) -> bool {
    let previous = text[..operator_idx]
        .chars()
        .rev()
        .find(|character| !character.is_whitespace());

    match previous {
        None => false,
        Some('(' | '+' | '-' | '*' | '/' | '%') => false,
        Some(_) => true,
    }
}

fn has_outer_parentheses(text: &str) -> bool {
    text.starts_with('(') && text.ends_with(')')
}

fn calculate_expression(left: i32, operator: char, right: i32) -> Result<i32> {
    let result = match operator {
        '+' => left.checked_add(right),
        '-' => left.checked_sub(right),
        '*' => left.checked_mul(right),
        '/' => {
            if right == 0 {
                bail!("division by zero");
            }

            left.checked_div(right)
        }
        '%' => {
            if right == 0 {
                bail!("remainder by zero");
            }

            left.checked_rem(right)
        }
        _ => bail!("unknown operator '{}'", operator),
    };

    result.context("expression result is out of i32 range")
}

fn parse_data_declaration(line: &str, symbols: &Symbols) -> Result<DataDeclaration> {
    let Some(open_bracket) = line.find('[') else {
        bail!("expected a data declaration in the form name[size] values");
    };

    let Some(close_offset) = line[open_bracket + 1..].find(']') else {
        bail!("data declaration is missing ']'");
    };

    let close_bracket = open_bracket + 1 + close_offset;
    let name = line[..open_bracket].trim();
    let size_text = line[open_bracket + 1..close_bracket].trim();
    let values = line[close_bracket + 1..].trim();

    if !is_symbol_name(name) {
        bail!("invalid data name '{name}'");
    }

    let size = parse_size_expression(size_text, symbols)?;

    if size <= 0 {
        bail!("data size must be greater than zero");
    }

    Ok(DataDeclaration {
        name: name.to_string(),
        size: size as usize,
        values: values.to_string(),
    })
}

fn parse_data_values(declaration: &DataDeclaration, symbols: &Symbols) -> Result<Vec<u16>> {
    if declaration.values.is_empty() {
        return Ok(vec![0; declaration.size]);
    }

    let mut values = Vec::new();

    for value_text in declaration.values.split(',') {
        let value_text = value_text.trim();

        if value_text.is_empty() {
            bail!("expected a value between commas");
        }

        let value = parse_expression(value_text, symbols)?;
        values.push(encode_data_word(value));
    }

    if values.len() == 1 {
        return Ok(vec![values[0]; declaration.size]);
    }

    if values.len() != declaration.size {
        bail!(
            "data declaration '{}' reserves {} words but provides {} values",
            declaration.name,
            declaration.size,
            values.len()
        );
    }

    Ok(values)
}

fn encode_data_word(value: i32) -> u16 {
    if value < i16::MIN as i32 || value > u16::MAX as i32 {
        eprintln!("warning: data value {value} does not fit in 16 bits");
    }

    value as u16
}

fn instruction_size(line: &str) -> usize {
    let op = line
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_ascii_uppercase();

    match op.as_str() {
        "LDI" => 2,
        _ => 1,
    }
}

fn parse_instruction(line: &str, pc: i32, symbols: &Symbols) -> Result<Vec<Instruction>> {
    let clean = line.replace(",", " ");
    let parts: Vec<&str> = clean.split_whitespace().collect();

    if parts.is_empty() {
        bail!("empty line");
    }

    let op = parts[0].to_ascii_uppercase();

    let instructions = match op.as_str() {
        "NOP" => {
            parse_no_operand(&parts)?;
            vec![Instruction::Nop]
        }
        "RET" => {
            parse_no_operand(&parts)?;
            vec![Instruction::Ret]
        }
        "MOV" => {
            let (rd, rs) = parse_rr(&parts)?;
            vec![Instruction::Mov { rd, rs }]
        }
        "CMP" => {
            let (rd, rs) = parse_rr(&parts)?;
            vec![Instruction::Cmp { rd, rs }]
        }
        "NOT" => {
            let (rd, rs) = parse_rr(&parts)?;
            vec![Instruction::Not { rd, rs }]
        }
        "ADD" => {
            let (rd, rs1, rs2) = parse_rrr(&parts)?;
            vec![Instruction::Add { rd, rs1, rs2 }]
        }
        "SUB" => {
            let (rd, rs1, rs2) = parse_rrr(&parts)?;
            vec![Instruction::Sub { rd, rs1, rs2 }]
        }
        "AND" => {
            let (rd, rs1, rs2) = parse_rrr(&parts)?;
            vec![Instruction::And { rd, rs1, rs2 }]
        }
        "OR" => {
            let (rd, rs1, rs2) = parse_rrr(&parts)?;
            vec![Instruction::Or { rd, rs1, rs2 }]
        }
        "XOR" => {
            let (rd, rs1, rs2) = parse_rrr(&parts)?;
            vec![Instruction::Xor { rd, rs1, rs2 }]
        }
        "LI" => {
            let (rd, imm8) = parse_ri8(&parts, symbols)?;
            vec![Instruction::Li { rd, imm8 }]
        }
        "LIH" => {
            let (rd, imm8) = parse_ri8(&parts, symbols)?;
            vec![Instruction::Lih { rd, imm8 }]
        }
        "ADDI" => {
            let (rd, imm8) = parse_ri8(&parts, symbols)?;
            vec![Instruction::Addi { rd, imm8 }]
        }
        "SUBI" => {
            let (rd, imm8) = parse_ri8(&parts, symbols)?;
            vec![Instruction::Subi { rd, imm8 }]
        }
        "CMPI" => {
            let (rd, imm8) = parse_ri8(&parts, symbols)?;
            vec![Instruction::Cmpi { rd, imm8 }]
        }
        "SLL" => {
            let (rd, rs, imm4) = parse_shift(&parts, symbols)?;
            vec![Instruction::Sll { rd, rs, imm4 }]
        }
        "SRL" => {
            let (rd, rs, imm4) = parse_shift(&parts, symbols)?;
            vec![Instruction::Srl { rd, rs, imm4 }]
        }
        "SRA" => {
            let (rd, rs, imm4) = parse_shift(&parts, symbols)?;
            vec![Instruction::Sra { rd, rs, imm4 }]
        }
        "LOAD" => {
            let (rd, rb, off5) = parse_load(&parts, symbols)?;
            vec![Instruction::Load { rd, rb, off5 }]
        }
        "STORE" => {
            let (rb, off5, rs) = parse_store(&parts, symbols)?;
            vec![Instruction::Store { rb, off5, rs }]
        }
        "JMP" => {
            let addr11 = parse_jump(&parts, symbols)?;
            vec![Instruction::Jmp { addr11 }]
        }
        "CALL" => {
            let addr11 = parse_jump(&parts, symbols)?;
            vec![Instruction::Call { addr11 }]
        }
        "BEQ" => {
            let off11 = parse_branch(&parts, pc, symbols)?;
            vec![Instruction::Beq { off11 }]
        }
        "BNE" => {
            let off11 = parse_branch(&parts, pc, symbols)?;
            vec![Instruction::Bne { off11 }]
        }
        "BLT" => {
            let off11 = parse_branch(&parts, pc, symbols)?;
            vec![Instruction::Blt { off11 }]
        }
        "BGT" => {
            let off11 = parse_branch(&parts, pc, symbols)?;
            vec![Instruction::Bgt { off11 }]
        }
        "BLE" => {
            let off11 = parse_branch(&parts, pc, symbols)?;
            vec![Instruction::Ble { off11 }]
        }
        "BGE" => {
            let off11 = parse_branch(&parts, pc, symbols)?;
            vec![Instruction::Bge { off11 }]
        }
        "LDI" => {
            expect_tokens(&parts, 3)?;
            let rd = parse_reg(parts[1])?;
            let value = parse_value(parts[2], symbols)?;
            warn_if_unsigned_truncates(value, 16, "imm16");

            vec![
                Instruction::Li {
                    rd,
                    imm8: (value & 0xFF) as u8,
                },
                Instruction::Lih {
                    rd,
                    imm8: ((value >> 8) & 0xFF) as u8,
                },
            ]
        }
        "CLR" => {
            expect_tokens(&parts, 2)?;
            let rd = parse_reg(parts[1])?;
            vec![Instruction::Li { rd, imm8: 0 }]
        }
        "INC" => {
            expect_tokens(&parts, 2)?;
            let rd = parse_reg(parts[1])?;
            vec![Instruction::Addi { rd, imm8: 1 }]
        }
        "DEC" => {
            expect_tokens(&parts, 2)?;
            let rd = parse_reg(parts[1])?;
            vec![Instruction::Subi { rd, imm8: 1 }]
        }
        _ => bail!("unknown instruction '{}'", parts[0]),
    };

    Ok(instructions)
}

fn parse_no_operand(parts: &[&str]) -> Result<()> {
    expect_tokens(parts, 1)
}

fn parse_rr(parts: &[&str]) -> Result<(Register, Register)> {
    expect_tokens(parts, 3)?;

    let rd = parse_reg(parts[1])?;
    let rs = parse_reg(parts[2])?;

    Ok((rd, rs))
}

fn parse_rrr(parts: &[&str]) -> Result<(Register, Register, Register)> {
    expect_tokens(parts, 4)?;

    let rd = parse_reg(parts[1])?;
    let rs1 = parse_reg(parts[2])?;
    let rs2 = parse_reg(parts[3])?;

    Ok((rd, rs1, rs2))
}

fn parse_ri8(parts: &[&str], symbols: &Symbols) -> Result<(Register, u8)> {
    expect_tokens(parts, 3)?;

    let rd = parse_reg(parts[1])?;
    let imm8 = parse_imm8(parts[2], symbols)?;

    Ok((rd, imm8))
}

fn parse_shift(parts: &[&str], symbols: &Symbols) -> Result<(Register, Register, u8)> {
    expect_tokens(parts, 4)?;

    let rd = parse_reg(parts[1])?;
    let rs = parse_reg(parts[2])?;
    let imm4 = parse_imm4(parts[3], symbols)?;

    Ok((rd, rs, imm4))
}

fn parse_load(parts: &[&str], symbols: &Symbols) -> Result<(Register, Register, i8)> {
    expect_tokens(parts, 3)?;

    let rd = parse_reg(parts[1])?;
    let (rb, off5) = parse_mem_operand(parts[2], symbols)?;

    Ok((rd, rb, off5))
}

fn parse_store(parts: &[&str], symbols: &Symbols) -> Result<(Register, i8, Register)> {
    expect_tokens(parts, 3)?;

    let (rb, off5) = parse_mem_operand(parts[1], symbols)?;
    let rs = parse_reg(parts[2])?;

    Ok((rb, off5, rs))
}

fn parse_jump(parts: &[&str], symbols: &Symbols) -> Result<u16> {
    expect_tokens(parts, 2)?;

    parse_addr11(parts[1], symbols)
}

fn parse_branch(parts: &[&str], pc: i32, symbols: &Symbols) -> Result<i16> {
    expect_tokens(parts, 2)?;

    parse_branch_off11(parts[1], pc, symbols)
}

fn expect_tokens(parts: &[&str], expected: usize) -> Result<()> {
    if parts.len() != expected {
        bail!("expected {expected} tokens, got {}", parts.len());
    }

    Ok(())
}

fn parse_reg(text: &str) -> Result<Register> {
    match text.to_ascii_uppercase().as_str() {
        "R0" => Ok(Register::R0),
        "R1" => Ok(Register::R1),
        "R2" => Ok(Register::R2),
        "R3" => Ok(Register::R3),
        "R4" => Ok(Register::R4),
        "R5" => Ok(Register::R5),
        "R6" => Ok(Register::R6),
        "R7" => Ok(Register::R7),
        _ => bail!("invalid register '{}'", text),
    }
}

fn parse_value(text: &str, symbols: &Symbols) -> Result<i32> {
    if is_number(text) {
        return parse_number(text);
    }

    if let Some(value) = parse_data_address(text, symbols)? {
        return Ok(value);
    }

    for symbol in symbols {
        if symbol.name == text {
            return Ok(symbol.value);
        }
    }

    bail!("undefined symbol '{}'", text)
}

fn parse_constant_value(text: &str, symbols: &Symbols) -> Result<i32> {
    if is_number(text) {
        return parse_number(text);
    }

    for symbol in symbols {
        if symbol.name == text {
            if symbol.kind != SymbolKind::Constant {
                bail!("data size can only reference constants");
            }

            return Ok(symbol.value);
        }
    }

    bail!("undefined constant '{}'", text)
}

fn parse_data_address(text: &str, symbols: &Symbols) -> Result<Option<i32>> {
    if !text.ends_with(')') {
        return Ok(None);
    }

    let Some(open_parenthesis) = text.find('(') else {
        return Ok(None);
    };

    let name = text[..open_parenthesis].trim();
    let index_text = text[open_parenthesis + 1..text.len() - 1].trim();

    if !is_symbol_name(name) {
        return Ok(None);
    }

    let Some(symbol) = symbols.iter().find(|symbol| symbol.name == name) else {
        bail!("undefined symbol '{name}'");
    };

    if symbol.kind != SymbolKind::Data {
        bail!("symbol '{name}' is not a data declaration");
    }

    let index = parse_expression(index_text, symbols)?;
    let Some(address) = symbol.value.checked_add(index) else {
        bail!("data address is out of range");
    };

    if !(0..DATA_MEM_SIZE).contains(&address) {
        bail!("data address {address} is out of range");
    }

    Ok(Some(address))
}

fn is_number(text: &str) -> bool {
    if text.is_empty() {
        return false;
    }

    let first_char = text.chars().next().unwrap();

    first_char.is_ascii_digit() || first_char == '-' || first_char == '+'
}

fn parse_number(text: &str) -> Result<i32> {
    let mut number_text = text;
    let mut negative = false;

    if number_text.starts_with('-') {
        negative = true;
        number_text = &number_text[1..];
    } else if number_text.starts_with('+') {
        number_text = &number_text[1..];
    }

    if number_text.is_empty() {
        bail!("invalid number '{}'", text);
    }

    let base;

    if number_text.starts_with("0x") || number_text.starts_with("0X") {
        base = 16;
        number_text = &number_text[2..];
    } else if number_text.starts_with("0b") || number_text.starts_with("0B") {
        base = 2;
        number_text = &number_text[2..];
    } else {
        base = 10;
    }

    if number_text.is_empty() {
        bail!("invalid number '{}'", text);
    }

    let parsed = match i64::from_str_radix(number_text, base) {
        Ok(value) => value,
        Err(_) => bail!("invalid number '{}'", text),
    };

    let value = if negative { -parsed } else { parsed };

    if value < i32::MIN as i64 || value > i32::MAX as i64 {
        bail!("number '{}' is out of range", text);
    }

    Ok(value as i32)
}

fn parse_imm8(text: &str, symbols: &Symbols) -> Result<u8> {
    let value = parse_value(text, symbols)?;

    warn_if_unsigned_truncates(value, 8, "imm8");

    Ok((value & 0xFF) as u8)
}

fn parse_imm4(text: &str, symbols: &Symbols) -> Result<u8> {
    let value = parse_value(text, symbols)?;

    warn_if_unsigned_truncates(value, 4, "imm4");

    Ok((value & 0xF) as u8)
}

fn parse_addr11(text: &str, symbols: &Symbols) -> Result<u16> {
    let value = parse_value(text, symbols)?;

    warn_if_unsigned_truncates(value, 11, "addr11");

    Ok((value & 0x7FF) as u16)
}

fn parse_branch_off11(text: &str, pc: i32, symbols: &Symbols) -> Result<i16> {
    let offset = if is_number(text) {
        parse_number(text)?
    } else {
        let target = parse_value(text, symbols)?;

        target - (pc + 1)
    };

    warn_if_signed_truncates(offset, 11, "off11");

    Ok(cut_signed(offset, 11) as i16)
}

fn parse_mem_operand(text: &str, symbols: &Symbols) -> Result<(Register, i8)> {
    if !text.ends_with(']') {
        bail!("memory operand '{}' must end with ']'", text);
    }

    let Some(bracket_idx) = text.find('[') else {
        bail!("memory operand '{}' must contain '['", text);
    };

    let offset_text = text[..bracket_idx].trim();
    let reg_text = text[bracket_idx + 1..text.len() - 1].trim();

    if reg_text.is_empty() {
        bail!("memory operand '{}' has no base register", text);
    }

    let rb = parse_reg(reg_text)?;
    let offset = if offset_text.is_empty() {
        0
    } else {
        parse_value(offset_text, symbols)?
    };

    warn_if_signed_truncates(offset, 5, "off5");

    Ok((rb, cut_signed(offset, 5) as i8))
}

fn warn_if_unsigned_truncates(value: i32, bits: u32, field: &str) {
    let max_value = (1_i32 << bits) - 1;

    if value < 0 || value > max_value {
        eprintln!("warning: {field} value {value} does not fit in {bits} bits");
    }
}

fn warn_if_signed_truncates(value: i32, bits: u32, field: &str) {
    let min_value = -(1_i32 << (bits - 1));
    let max_value = (1_i32 << (bits - 1)) - 1;

    if value < min_value || value > max_value {
        eprintln!("warning: {field} value {value} does not fit in {bits} bits");
    }
}

fn cut_signed(value: i32, bits: u32) -> i32 {
    let mask = (1_i32 << bits) - 1;
    let sign_bit = 1_i32 << (bits - 1);
    let cut_value = value & mask;

    if cut_value & sign_bit != 0 {
        cut_value - (1_i32 << bits)
    } else {
        cut_value
    }
}
