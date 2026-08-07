# AthenISA Language Support

Visual Studio Code language support for AthenISA `.athe` assembly files.

## Features

- Highlighting for instructions and pseudo-instructions
- Highlighting for sections, registers, numbers, labels, constants, data names, and symbols
- Line comments beginning with `;`
- Bracket and parenthesis matching

The extension recognizes registers from `R0` to `R7` and decimal, hexadecimal, and binary numbers.

## Installation

When available from the Visual Studio Marketplace, install `AthenISA Language Support` from the Extensions view in Visual Studio Code.

To install a downloaded `.vsix` package, open the Extensions view, select `Install from VSIX...` from its actions menu, and choose the package.

> [!NOTE]
> Extensions installed manually from a VSIX package do not receive automatic Marketplace updates.

## Development

### Test locally

Launch a separate Visual Studio Code window with the local extension enabled:

```sh
code --extensionDevelopmentPath=/path/to/athenisa/vscode /path/to/program.athe
```

### Build a VSIX package

Node.js and npm are only required to package or publish the extension. From the `vscode` directory, run:

```sh
npx @vscode/vsce package
```

This creates `athenisa-language-X.Y.Z.vsix`. Update the `version` field in `package.json` before creating a new release.

### Publish

Publishing requires a Marketplace publisher matching the `publisher` field in `package.json`. Build the `.vsix`, open the [Visual Studio Marketplace](https://marketplace.visualstudio.com/manage/publishers/), and select **New extension > Visual Studio Code** to upload it. For updates, increment the extension version and upload a new package.
