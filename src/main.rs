mod directory_scanner;
mod html_extractor;
mod lexer;
mod tfidf_calculator;

use std::collections::HashMap;
use std::io;
use std::path::Path;

use directory_scanner::scan_directory_recursive;
use html_extractor::extract_html_text;
use tfidf_calculator::calculate_tfidf;

fn find_best_matching_results(
    tfidf_scores: &HashMap<String, HashMap<String, f64>>,
    query: &str,
    top_results: usize,
) {
    let query = query.to_lowercase();

    for (doc_id, tf) in tfidf_scores.iter() {
        let mut matching_terms: Vec<(&String, &f64)> = tf
            .iter()
            .filter(|(term, _)| term.to_lowercase().contains(&query))
            .collect();
        matching_terms.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

        println!("Best matching results for Document {} with query '{}':", doc_id, query);

        for (term, score) in matching_terms.iter().take(top_results) {
            println!("Term: {} has a TF-IDF Score of {}", term, score);
        }
    }
}

fn find_most_frequent_terms(
    tfidf_scores: &HashMap<String, HashMap<String, f64>>,
) -> HashMap<String, String> {
    let mut most_frequent_terms = HashMap::new();

    for (doc_id, tf) in tfidf_scores.iter() {
        let mut sorted_terms: Vec<(&String, &f64)> = tf.iter().collect();
        sorted_terms.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

        let mut terms = Vec::new();

        for (term, score) in sorted_terms {
            terms.push(format!("Term: {} has a TF-IDF Score of {}", term, score));
        }

        most_frequent_terms.insert(doc_id.to_string(), terms.join("\n"));
    }

    most_frequent_terms
}

fn main() {
    let root_directory_path = "./examples/html";

    let mut texts = Vec::new();

    let mut callback = |file_path: &Path| {
        println!("Processing file: {:?}", file_path.display());

        let text = extract_html_text(&file_path.to_str().unwrap().to_owned());

        texts.push(text);
    };

    scan_directory_recursive(root_directory_path, &mut callback);

    let tfidf_scores = calculate_tfidf(&texts);

    let mut query: String = String::new();
    println!("Enter a query string: ");
    io::stdin().read_line(&mut query).expect("Failed to read input");
    query = query.trim().to_string();

    let mut top_results = String::new();
    println!("Enter the number of top matching results to display: ");
    io::stdin().read_line(&mut top_results).expect("Failed to read input");
    let top_results = top_results.trim().parse::<usize>().unwrap_or(10);

    find_best_matching_results(&tfidf_scores, &query, top_results);
}
