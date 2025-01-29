use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure a file path is provided
    if args.len() != 2 {
        eprintln!("Usage: smart_file_analyzer <file_path>");
        process::exit(1);
    }

    let file_path = &args[1];

    // Open the file
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            process::exit(1);
        }
    };

    // Read the file and process text
    let reader = BufReader::new(file);
    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;
    let mut word_map: HashMap<String, usize> = HashMap::new();
    let mut longest_word = String::new();

    for line in reader.lines() {
        let line = match line {
            Ok(content) => content,
            Err(_) => {
                // Skip invalid UTF-8 lines
                continue;
            }
        };

        line_count += 1;
        char_count += line.chars().count();

        let words: Vec<&str> = line.split_whitespace().collect();
        word_count += words.len();

        for &word in &words {
            let clean_word = word.to_lowercase(); // Convert to lowercase for accurate counting
            *word_map.entry(clean_word.clone()).or_insert(0) += 1;

            if word.len() > longest_word.len() {
                longest_word = word.to_string();
            }
        }
    }

    // Find the most common word
    let most_common_word = word_map.iter().max_by_key(|entry| entry.1);

    // Unique word count
    let unique_word_count = word_map.len();

    // Display results
    println!("📊 Smart File Analyzer Results for: {}", file_path);
    println!("---------------------------------");
    println!("📄 Total Lines: {}", line_count);
    println!("📝 Total Words: {}", word_count);
    println!("🔠 Total Characters: {}", char_count);
    println!("📏 Longest Word: {}", longest_word);
    if let Some((word, count)) = most_common_word {
        println!("🏆 Most Common Word: \"{}\" ({} times)", word, count);
    } else {
        println!("🏆 Most Common Word: None");
    }
    println!("🔣 Unique Words: {}", unique_word_count);
}

