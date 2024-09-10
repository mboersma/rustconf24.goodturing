use wasm_bindgen::prelude::*;

use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

use lazy_regex::regex;

fn good_turing<R: BufRead>(reader: R) -> Result<(u32, u32), io::Error> {
    let re = regex!(r"\b\w+\b");
    let mut word_to_count_even = HashMap::new();
    let mut word_set_odd = HashSet::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        // split into words with regex
        for word in re.find_iter(&line) {
            let word = word.as_str();
            if index % 2 == 0 {
                // count # of times a word occurs on the even lines
                *word_to_count_even.entry(word.to_string()).or_insert(0) += 1;
            } else {
                // odd lines
                word_set_odd.insert(word.to_string());
            }
        }
    }

    let singleton_count_even = word_to_count_even
        .iter()
        .filter(|(_, &count)| count == 1)
        .count() as u32;
    let only_odd_count = word_set_odd
        .into_iter()
        .filter(|word| !word_to_count_even.contains_key(word))
        .count() as u32;

    Ok((singleton_count_even, only_odd_count))
}

#[wasm_bindgen]
pub fn good_turing_js(data: &[u8]) -> Result<Vec<u32>, String> {
    let reader = BufReader::new(data);
    match good_turing(reader) {
        Ok((prediction, actual)) => Ok(vec![prediction, actual]),
        Err(e) => Err(format!("Error processing data: {e}")),
    }
}

// fn main() {
//     let file_name = env::args().nth(1).expect("no file name given");
//     let (singleton_count_even, only_odd_count) = good_turing(&file_name).unwrap();

//     println!(
//         "Prediction (words that appear exactly once on even lines): {}",
//         singleton_count_even
//     );
//     println!(
//         "Actual distinct words that appear only on odd lines: {}",
//         only_odd_count
//     );
// }

// test `process_file` on `./pg100.txt`. The answer is 10223 and 7967.
#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    use super::*;

    use std::fs::File;

    #[test]
    fn test_process_file() {
        let reader = BufReader::new(File::open("pg100.txt").unwrap());
        let (prediction, actual) = good_turing(reader).unwrap();
        assert_eq!(prediction, 10223);
        assert_eq!(actual, 7967);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_good_turing_js() {
        let data = include_bytes!("../pg100.txt");
        let result = good_turing_js(data).unwrap();
        assert_eq!(result, vec![10223, 7967]);
    }
}
