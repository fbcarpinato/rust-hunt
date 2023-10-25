use crate::lexer::Lexer;
use std::collections::{HashMap, HashSet};

pub fn calculate_tfidf(texts: &[String]) -> HashMap<String, HashMap<String, f64>> {
    let mut tfidf_scores: HashMap<String, HashMap<String, f64>> = HashMap::new();
    let mut document_frequencies = HashMap::new();

    for (i, text) in texts.iter().enumerate() {
        let mut tf: HashMap<String, f64> = HashMap::new();
        let mut term_count = 0;

        let lexer = Lexer::new(text);
        let mut seen = HashSet::new();

        for term in lexer {
            *tf.entry(term.to_string()).or_insert(0.0) += 1.0;
            term_count += 1;

            if seen.insert(term.to_string()) {
                *document_frequencies.entry(term.to_string()).or_insert(0) += 1;
            }
        }

        let num_terms = if term_count > 0 {
            term_count as f64
        } else {
            1.0
        };

        for (_, count) in tf.iter_mut() {
            *count /= num_terms;
        }

        tfidf_scores.insert(i.to_string(), tf);
    }

    for (doc_id, tf) in tfidf_scores.iter_mut() {
        for (term, count) in tf.iter_mut() {
            let document_frequency = document_frequencies.get(term).unwrap_or(&0);
            let idf = (texts.len() as f64 / (*document_frequency as f64 + 1.0)).ln();
            *count *= idf;
        }
    }

    tfidf_scores
}
