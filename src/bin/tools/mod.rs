use std::fs;


pub fn read_file_to_string(filename: &str) -> String {
    let path = format!("./files/{}", filename);
    fs::read_to_string(path).unwrap()
}

pub fn read_file_to_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    let path = format!("./files/{}", filename);
    for line in fs::read_to_string(path).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

