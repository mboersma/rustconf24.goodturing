use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

use lazy_regex::regex;

fn good_turning(file_name: &str) -> Result<(usize, usize), io::Error> {
    let reader = BufReader::new(File::open(file_name)?);

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
        .count();
    let only_odd_count = word_set_odd
        .into_iter()
        .filter(|word| !word_to_count_even.contains_key(word))
        .count();

    Ok((singleton_count_even, only_odd_count))
}
fn main() {
    let file_name = env::args().nth(1).expect("no file name given");
    let (singleton_count_even, only_odd_count) = good_turning(&file_name).unwrap();

    println!(
        "Prediction (Words that appear exactly once on even lines): {}",
        singleton_count_even
    );
    println!(
        "Actual distinct words that appear only on odd lines: {}",
        only_odd_count
    );
}

// test `process_file` on `./pg100.txt`. The answer is 10223 and 7967.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_file() {
        let (prediction, actual) = good_turning("./pg100.txt").unwrap();
        assert_eq!(prediction, 10223);
        assert_eq!(actual, 7967);
    }
}
