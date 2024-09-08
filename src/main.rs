use std::fs;
use std::path::Path;
use pulldown_cmark::{Parser, html};
use walkdir::WalkDir;

fn main() {
    // Define directories
    let input_dir = "content";
    let output_dir = "public";
    let styles_dir = "styles";

    // Create the output directory if it doesn't exist
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    // Copy the CSS file to the output directory
    let css_source = Path::new(styles_dir).join("style.css");
    let css_destination = Path::new(output_dir).join("style.css");
    fs::copy(&css_source, &css_destination).expect("Failed to copy CSS file");

    // Process each Markdown file
    for entry in WalkDir::new(input_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file() && e.path().extension().and_then(|ext| ext.to_str()) == Some("md"))
    {
        process_markdown(entry.path(), output_dir);
    }

    println!("Site generation complete!");
}

fn process_markdown(input_path: &Path, output_dir: &str) {
    // Read the Markdown content
    let markdown_content = fs::read_to_string(input_path).expect("Failed to read Markdown file");

    // Parse Markdown to HTML
    let parser = Parser::new(&markdown_content);
    let mut html_content = String::new();
    html::push_html(&mut html_content, parser);

    // Extract the title from the filename
    let title = input_path.file_stem().and_then(|s| s.to_str()).unwrap_or("Untitled");

    // Create the HTML template with a link to the CSS file
    let html_template = format!(
        "<!DOCTYPE html>
        <html>
        <head>
            <meta charset=\"UTF-8\">
            <title>{title}</title>
            <link rel=\"stylesheet\" href=\"style.css\">
        </head>
        <body>
            {content}
        </body>
        </html>",
        title = title,
        content = html_content
    );

    // Define the output path (change .md to .html)
    let output_filename = format!("{}.html", title);
    let output_path = Path::new(output_dir).join(output_filename);

    // Write the HTML content to the output file
    fs::write(&output_path, html_template).expect("Failed to write HTML file");
}
