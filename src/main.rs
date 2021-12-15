//Programmer: Alexander Leones 
//Date: 2021-12-14
// This program returns an exhaustive list of english words that can be formed 
// using no-more and no-less than every letter in a provided string.
/////// Credits: ///////////
//find_all_words alogorithm was first published by Pham Trung at:
// https://stackoverflow.com/a/25298960
// dictionary txt file can be found at https://github.com/dwyl/english-words

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!(
        "{:?}",
        find_valid_words(
            /*replace following string with scrambled word you wish to unscramble. */
            "iverdfdsdafefrgsfcrervrs"
        )
    );
}

fn find_valid_words(letters: &str) -> Vec<String> {
    let mut avail = vec![0_usize; 26];
    // store the frequency of each letter in the string.
    for c in letters.chars() {
        let index = c as usize - 'a' as usize;
        avail[index] += 1;
    }

    // create a vector to store the result.
    let mut result: Vec<String> = Vec::new();

    // open the dictionary file and create an input buffer.
    let filename = "src/words_alpha.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // for each word in the dictionary..
    for word in reader.lines() {
        let mut count = vec![0; 26];
        let mut ok = true;
        let word = word.unwrap();
        /*
         * count the frequency of each character in the dictionary word.
         * set ok to false and break if too many of
         * a letter is found in the dictionary word.
         */
        for c in word.chars() {
            let index = c as usize - 'a' as usize;
            count[index] += 1;
            if count[index] > avail[index] {
                ok = false;
                break;
            }
        }
        // if word has the right amount of each letter, add to result vector.
        if ok && (word.len() == letters.len()) {
            result.push(word);
        }
    }
    result
}
