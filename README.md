# Minigrep

Minigrep is a minimalistic text search tool written in Rust. It allows you to search for a query string within a specified file, similar to the Unix `grep` command. 

## Features

- **Case-sensitive and case-insensitive search**: By default, the search is case-sensitive. However, you can enable case-insensitive search by passing "true" as a fourth argument or setting the `IGNORE_CASE` environment variable.

- **Command-line interface**: Minigrep is designed to be used from the command line. You can pass the query string and file path as command-line arguments.

## Usage

To use Minigrep, pass the query string and file path as command-line arguments:

```bash
cargo run <query> <file_path>
```

To enable case-insensitive search, pass "true" as a third argument or set the `IGNORE_CASE` environment variable:

```bash
cargo run <query> <file_path> true
```

or

```bash
IGNORE_CASE=1 cargo run <query> <file_path>
```

## Error Handling

Minigrep provides clear error messages for common issues, such as insufficient arguments or problems with file reading. If an error occurs, Minigrep will print an error message and exit with a status code of 1.

## Testing

Minigrep includes unit tests for the `search` and `search_case_insensitive` functions. You can run these tests with `cargo test`.

## License

Minigrep is open-source software

## Installation

1. **Install Rust**: If you haven't already, [install Rust](https://www.rust-lang.org/tools/install) on your machine.

2. **Clone the repository**: Clone the Minigrep repository to your local machine. You can do this by running the following command in your terminal:
```bash
git clone https://github.com/tomily1/minigrep.git
```

3. Navigate to the project directory: Change your current directory to the Minigrep directory:
```bash
cd minigrep
```
4. Build the project: Build the Minigrep project using Cargo, Rust's package manager:
```bash
cargo build --release
```
The --release flag will create an optimized build.

5. Run Minigrep: You can now run Minigrep with the following command:
```bash
cargo run <query> <file_path>
```
Replace <query> and <file_path> with your actual query and file path.
