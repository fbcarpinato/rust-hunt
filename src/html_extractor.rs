use std::fs;
use tl;

pub fn extract_html_text(file_path: &str) -> String {
    let file_content = fs::read_to_string(file_path).expect("File not found");

    let dom = tl::parse(&file_content, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();

    let handle = dom
        .query_selector("div")
        .and_then(|mut iter| iter.next())
        .unwrap();
    let node = handle.get(dom.parser()).unwrap();

    return node.inner_text(parser).to_string();
}
