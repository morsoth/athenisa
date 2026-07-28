use anyhow::{Result, bail};

pub fn words_to_hex(words: &[u16]) -> String {
    let mut hex = String::new();

    for word in words {
        hex.push_str(&format!("{word:04X}\n"));
    }

    hex
}

pub fn hex_to_words(text: &str) -> Result<Vec<u16>> {
    let mut words = Vec::new();

    for (line_index, raw_line) in text.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.trim();

        if line.is_empty() {
            continue;
        }

        if line.len() > 4 {
            bail!("line {line_number}: hexadecimal word is longer than 4 digits");
        }

        let word = match u16::from_str_radix(line, 16) {
            Ok(word) => word,
            Err(_) => bail!("line {line_number}: invalid hexadecimal word '{line}'"),
        };

        words.push(word);
    }

    Ok(words)
}

pub fn words_to_bytes(words: &[u16]) -> Vec<u8> {
    let mut bytes = Vec::new();

    for word in words {
        bytes.push((*word & 0x00FF) as u8);
        bytes.push((*word >> 8) as u8);
    }

    bytes
}

pub fn bytes_to_words(bytes: &[u8]) -> Result<Vec<u16>> {
    if !bytes.len().is_multiple_of(2) {
        bail!("binary image contains an incomplete 16-bit word");
    }

    let mut words = Vec::new();

    for pair in bytes.chunks_exact(2) {
        words.push(u16::from_le_bytes([pair[0], pair[1]]));
    }

    Ok(words)
}
