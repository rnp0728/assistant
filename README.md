# Assistant CLI

A command-line tool to read and format file contents, with clipboard support.

## Features

- Read and format content from multiple files and folders
- Process files and directories in the same command
- Filter out specific file extensions
- Copy formatted content to clipboard
- Append content to existing clipboard
- Show current clipboard contents
- Cross-platform support (macOS, Windows, Linux)

## Installation

### From crates.io (Recommended)

```bash
cargo install assistant-cli
```

### From Source

1. Clone the repository:
```bash
git clone https://github.com/rnp0728/assistant.git
cd assistant
```

2. Build and install:
```bash
cargo install --path .
```

## Usage

### Basic Usage

```bash
# Process specific files
assistant file1.txt file2.txt

# Process a folder
assistant /path/to/folder

# Process files and folders together
assistant file1.txt /path/to/folder file2.txt

# Exclude specific extensions
assistant -e .git -e .DS_Store /path/to/folder

# Copy to clipboard
assistant -c file1.txt file2.txt

# Append to clipboard
assistant -a file1.txt file2.txt

# Show clipboard contents
assistant --show-clipboard
```

### Examples

#### macOS/Linux
```bash
# Process multiple files
assistant src/main.rs src/cli.rs

# Process a directory
assistant src/

# Process files and directories together
assistant src/main.rs src/features/ src/cli.rs

# Exclude specific extensions
assistant -e .git -e .DS_Store src/

# Copy to clipboard
assistant -c src/main.rs src/cli.rs

# Append to clipboard
assistant -a src/main.rs src/cli.rs

# Show clipboard contents
assistant --show-clipboard
```

#### Windows
```powershell
# Process multiple files
assistant src\main.rs src\cli.rs

# Process a directory
assistant src\

# Process files and directories together
assistant src\main.rs src\features\ src\cli.rs

# Exclude specific extensions
assistant -e .git -e .DS_Store src\

# Copy to clipboard
assistant -c src\main.rs src\cli.rs

# Append to clipboard
assistant -a src\main.rs src\cli.rs

# Show clipboard contents
assistant --show-clipboard
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.