mod directory_scanner;
mod lexer;
mod html_extractor;


use std::path::Path;

use lexer::Lexer;
use directory_scanner::scan_directory_recursive;
use html_extractor::extract_html_text;

fn main() {
    let root_directory_path = "./examples/html";

    let callback = |file_path: &Path| {
        println!("Processing file: {:?}", file_path.display());

        let text = extract_html_text(&file_path.to_str().unwrap().to_owned());
        let lexer = Lexer::new(&text);
        println!("Tokens: {:?}", lexer.count());
    };

    scan_directory_recursive(root_directory_path, &callback);
}
