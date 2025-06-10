# Assistant CLI

A powerful command-line tool for file operations and content management. Currently supports file content extraction with clipboard integration.

## Features

- Extract and format content from multiple files and folders
- Process files and directories in the same command
- Filter out specific file extensions
- Copy formatted output to clipboard
- Append content to existing clipboard content
- Show current clipboard contents
- Cross-platform support (macOS, Windows, Linux)

## Installation

### Prerequisites

#### For All Platforms
- Rust and Cargo installed on your system
  ```bash
  # macOS/Linux
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

  # Windows (PowerShell)
  winget install Rustlang.Rustup
  # OR
  Invoke-WebRequest -Uri https://win.rustup.rs -OutFile rustup-init.exe
  .\rustup-init.exe
  ```

### Platform-Specific Installation

#### macOS
1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/assistant.git
   cd assistant
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Install globally (choose one method):

   **Option A: Install to system path (requires sudo)**
   ```bash
   sudo cp ./target/release/assistant /usr/local/bin/
   sudo chmod +x /usr/local/bin/assistant
   ```

   **Option B: Install to user's Cargo bin**
   ```bash
   cp ./target/release/assistant ~/.cargo/bin/
   chmod +x ~/.cargo/bin/assistant
   ```

#### Linux
1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/assistant.git
   cd assistant
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Install globally (choose one method):

   **Option A: Install to system path (requires sudo)**
   ```bash
   sudo cp ./target/release/assistant /usr/local/bin/
   sudo chmod +x /usr/local/bin/assistant
   ```

   **Option B: Install to user's Cargo bin**
   ```bash
   cp ./target/release/assistant ~/.cargo/bin/
   chmod +x ~/.cargo/bin/assistant
   ```

#### Windows
1. Clone the repository:
   ```powershell
   git clone https://github.com/yourusername/assistant.git
   cd assistant
   ```

2. Build the project:
   ```powershell
   cargo build --release
   ```

3. Install globally (choose one method):

   **Option A: Install to system path (requires admin)**
   ```powershell
   # Create a directory for your tools (if it doesn't exist)
   New-Item -ItemType Directory -Force -Path "C:\Tools"
   
   # Copy the binary
   Copy-Item .\target\release\assistant.exe C:\Tools\
   
   # Add to PATH (run in admin PowerShell)
   [Environment]::SetEnvironmentVariable(
       "Path",
       [Environment]::GetEnvironmentVariable("Path", "Machine") + ";C:\Tools",
       "Machine"
   )
   ```

   **Option B: Install to user's Cargo bin**
   ```powershell
   Copy-Item .\target\release\assistant.exe ~\.cargo\bin\
   ```

### Verify Installation

#### macOS/Linux
```bash
which assistant
# Should show the path to your installation
assistant --version
```

#### Windows
```powershell
Get-Command assistant
# Should show the path to your installation
assistant --version
```

## Usage

### Show Clipboard Contents

#### macOS/Linux
```bash
# Long form
assistant --show-clipboard
# Short form
assistant -s
```

#### Windows
```powershell
# Long form
assistant.exe --show-clipboard
# Short form
assistant.exe -s
```

### Basic Usage

#### macOS/Linux
```bash
# Process specific files
assistant file1.txt file2.txt

# Process a folder
assistant /path/to/folder

# Process multiple files and folders together
assistant file1.txt /path/to/folder file2.txt
assistant src/ Cargo.toml README.md

# Exclude specific file extensions
assistant -e .git -e .DS_Store /path/to/folder
```

#### Windows
```powershell
# Process specific files
assistant.exe file1.txt file2.txt

# Process a folder
assistant.exe C:\path\to\folder

# Process multiple files and folders together
assistant.exe file1.txt C:\path\to\folder file2.txt
assistant.exe src\ Cargo.toml README.md

# Exclude specific file extensions
assistant.exe -e .git -e .DS_Store C:\path\to\folder
```

### Copy to Clipboard

#### macOS/Linux
```bash
# Long form
assistant --copy /path/to/folder
# Short form
assistant -c /path/to/folder
```

#### Windows
```powershell
# Long form
assistant.exe --copy C:\path\to\folder
# Short form
assistant.exe -c C:\path\to\folder
```

### Append to Clipboard

#### macOS/Linux
```bash
# Long form
assistant --append /path/to/folder
# Short form
assistant -a /path/to/folder
```

#### Windows
```powershell
# Long form
assistant.exe --append C:\path\to\folder
# Short form
assistant.exe -a C:\path\to\folder
```

### Examples

1. Show clipboard contents:
   ```bash
   # macOS/Linux
   assistant -s

   # Windows
   assistant.exe -s
   ```

2. Process a folder:
   ```bash
   # macOS/Linux
   assistant /path/to/folder

   # Windows
   assistant.exe C:\path\to\folder
   ```

3. Process multiple files and folders together:
   ```bash
   # macOS/Linux
   assistant src/ Cargo.toml README.md
   assistant file1.txt /path/to/folder file2.txt

   # Windows
   assistant.exe src\ Cargo.toml README.md
   assistant.exe file1.txt C:\path\to\folder file2.txt
   ```

4. Exclude specific file extensions:
   ```bash
   # macOS/Linux
   assistant -e .git -e .DS_Store /path/to/folder

   # Windows
   assistant.exe -e .git -e .DS_Store C:\path\to\folder
   ```

5. Copy mixed content to clipboard:
   ```bash
   # macOS/Linux
   assistant -c src/ Cargo.toml

   # Windows
   assistant.exe -c src\ Cargo.toml
   ```

6. Append mixed content to clipboard:
   ```bash
   # macOS/Linux
   assistant -a src/ Cargo.toml

   # Windows
   assistant.exe -a src\ Cargo.toml
   ```

## Output Format

The tool formats the output as follows:
```
FILE NAME: filename.txt
FILE CONTENT
```
```
[file contents here]
```

---

## Development

### Project Structure
```
src/
├── main.rs           # Main application entry point
├── cli.rs           # CLI argument handling
├── features/        # Feature modules
│   ├── mod.rs
│   └── file_formatter.rs  # File formatting feature
└── utils/           # Utility modules
    ├── mod.rs
    └── clipboard.rs       # Clipboard operations
```

### Building for Development
```bash
# macOS/Linux
cargo build

# Windows
cargo build
```

### Running Tests
```bash
# macOS/Linux
cargo test

# Windows
cargo test
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with Rust
- Uses [clap](https://github.com/clap-rs/clap) for CLI argument parsing
- Uses [arboard](https://github.com/1Password/arboard) for clipboard operations 