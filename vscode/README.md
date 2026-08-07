# AthenISA Language Support

Visual Studio Code language support for AthenISA `.athe` assembly files.

## Features

- Highlighting for instructions and pseudo-instructions
- Highlighting for registers, numbers, labels, constants, and symbols
- Line comments beginning with `;`
- Bracket and parenthesis matching

The extension recognizes registers from `R0` to `R7` and decimal, hexadecimal, and binary numbers.

## Installation

When available from the Visual Studio Marketplace, install `AthenISA Language Support` from the Extensions view in Visual Studio Code.

To install a downloaded `.vsix` package, open the Extensions view, select `Install from VSIX...` from its actions menu, and choose the package. It can also be installed from a terminal:

```sh
code --install-extension athenisa-language-0.1.0.vsix
```

Once installed, the extension is automatically enabled for every `.athe` file. It can be enabled, disabled, or uninstalled from the Extensions view like any other Visual Studio Code extension.

> [!NOTE]
> Extensions installed manually from a VSIX package do not receive automatic Marketplace updates.

## Development

### Test locally

Launch a separate Visual Studio Code window with the local extension enabled:

```sh
code --extensionDevelopmentPath=/path/to/athenisa/vscode /path/to/program.athe
```

After changing the grammar, run `Developer: Reload Window` from the Command Palette in the development window. Use `Developer: Inspect Editor Tokens and Scopes` to inspect the highlighting assigned to a token.

### Build a VSIX package

Node.js and npm are only required to package or publish the extension. From the `vscode` directory, run:

```sh
npx @vscode/vsce package
```

For version `0.1.0`, this creates `athenisa-language-0.1.0.vsix`. Update the `version` field in `package.json` before creating a new release.

To reinstall a rebuilt package with the same version, use:

```sh
code --install-extension athenisa-language-0.1.0.vsix --force
```

### Publish

Publishing to the Visual Studio Marketplace requires a Marketplace publisher matching the `publisher` field in `package.json`:

```sh
npx @vscode/vsce publish
```

Alternatively, the generated `.vsix` can be attached to a GitHub Release for manual installation.
