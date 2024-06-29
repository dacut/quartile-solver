use std::{
    collections::{BTreeSet, HashSet},
    env::args,
    process::ExitCode,
};

const ENGLISH_WORDS: &str = include_str!("../words.txt");

fn main() -> ExitCode {
    let words = args().skip(1).collect::<Vec<_>>();
    if words.is_empty() {
        eprintln!("Usage: quartile-solver <word> ...");
        return ExitCode::FAILURE;
    }

    run(&words);
    ExitCode::SUCCESS
}

fn english_words() -> HashSet<&'static str> {
    let mut words = HashSet::new();
    for word in ENGLISH_WORDS.split_whitespace() {
        words.insert(word);
    }
    words
}

fn run<S: AsRef<str>>(words: &[S]) {
    let english_words = english_words();
    let mut results = BTreeSet::new();

    for (i, word_i_string) in words.iter().enumerate() {
        let word_i = word_i_string.as_ref();
        if english_words.contains(word_i) {
            results.insert(word_i.to_string());
        }

        for (j, word_j_string) in words.iter().enumerate() {
            if i == j {
                continue;
            }

            let word_j = word_j_string.as_ref();
            let combined = format!("{}{}", word_i, word_j);
            if english_words.contains(combined.as_str()) {
                results.insert(combined);
            }

            for (k, word_k_string) in words.iter().enumerate() {
                if i == k || j == k {
                    continue;
                }

                let word_k = word_k_string.as_ref();
                let combined = format!("{}{}{}", word_i, word_j, word_k);
                if english_words.contains(combined.as_str()) {
                    results.insert(combined);
                }

                for (l, word_l_string) in words.iter().enumerate() {
                    if i == l || j == l || k == l {
                        continue;
                    }

                    let word_l = word_l_string.as_ref();
                    let combined = format!("{}{}{}{}", word_i, word_j, word_k, word_l);
                    if english_words.contains(combined.as_str()) {
                        results.insert(combined);
                    }
                }
            }
        }
    }

    for result in results.iter() {
        println!("{result}");
    }
}
