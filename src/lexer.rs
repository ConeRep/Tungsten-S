use std::fs::File;
use std::io::{self, BufRead, BufReader, Error};

fn find_col(line: &str, mut col: usize, predicate: impl Fn(char) -> bool) -> usize {
    while col < line.len() && !predicate(line.chars().nth(col).unwrap()) {
        col += 1;
    }
    col
}

fn strip_col(line: &str, mut col: usize) -> usize {
    while col < line.len() && line.chars().nth(col).unwrap().is_whitespace() {
        col += 1;
    }
    col
}

fn chop_word(line: &str, mut col: usize) -> usize {
    while col < line.len() && !line.chars().nth(col).unwrap().is_whitespace() {
        col += 1;
    }
    col
}

// Example function to tokenize a line
fn lex_line(line: &str) -> Vec<String> {
    line.split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

pub fn lex_file(file_path: &str) -> Result<Vec<(String, usize, usize, String)>, std::io::Error> {
    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);
    let mut result = Vec::new();

    for (row, line) in reader.lines().enumerate() {
        let line = line?;
        // Split the line by "//" and get the first part
        let parts: Vec<&str> = line.split("//").collect();
        // Convert &String to &str for compatibility
        let line_content = parts.get(0).unwrap_or(&line.as_str()).trim();

        // Tokenize the line
        let tokens = lex_line(line_content);
        for (col, token) in tokens.into_iter().enumerate() {
            result.push((file_path.to_string(), row + 1, col + 1, token));
        }
    }

    Ok(result)
}