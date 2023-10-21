use std::fs;
use tl;

mod lexer;

use lexer::Lexer;

fn extract_html_text(file_path: &str) -> String {
    let file_content = fs::read_to_string(file_path).expect("File not found");

    let dom = tl::parse(&file_content, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();

    let handle = dom
        .query_selector("html")
        .and_then(|mut iter| iter.next())
        .unwrap();
    let node = handle.get(dom.parser()).unwrap();

    return node.inner_text(parser).to_string();
}

fn main() {
    let file_path: &str = "./examples/html/example1.html";

    let text = extract_html_text(file_path);

    let lexer = Lexer::new(&text);

    for token in lexer {
        println!("token: {token}");
    }
}
