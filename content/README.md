# Rustysite

![Rustysite](https://github.com/user-attachments/assets/e587d6d4-db12-442a-972d-f54ea1af7137)

Rustysite is a simple static site generator written in Rust. It converts Markdown files to HTML and provides a basic structure for creating a static website.

## Features

- Converts Markdown files to HTML
- Automatically generates an HTML structure with proper metadata
- Copies a CSS file for styling
- Processes all Markdown files in the input directory recursively

## Prerequisites

Before you begin, ensure you have Rust and Cargo installed on your system. You can install them from [https://www.rust-lang.org/](https://www.rust-lang.org/).

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/rustysite.git
   cd rustysite
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

1. Create a `content` directory in the project root and add your Markdown files there.

2. Create a `styles` directory and add a `style.css` file for your custom styles.

3. Run the program:
   ```bash
   cargo run --release
   ```

4. The generated HTML files will be in the `public` directory.

## Project Structure

- `src/main.rs`: The main Rust source file containing the site generation logic
- `content/`: Directory for your Markdown files
- `styles/`: Directory for your CSS files
- `public/`: Output directory for generated HTML files

## Dependencies

This project uses the following Rust crates:

- `pulldown-cmark`: For parsing Markdown and converting it to HTML
- `walkdir`: For recursively walking through directories

## Customization

You can customize the HTML template and CSS styling by modifying the `process_markdown` function in `main.rs` and updating the `style.css` file in the `styles` directory.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [MIT License](LICENSE).