//Programmer: Alexander Leones
//Date: 2021-12-14
// This program returns an exhaustive list of english words that can be formed
// using no-more and no-less than every letter in a provided string.
/////// Credits: ///////////
//find_valid_words alogorithm was first published by Pham Trung at:
// https://stackoverflow.com/a/25298960
// dictionary txt file can be found at https://github.com/dwyl/english-words

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // let mut scrambled_word: String;
    // loop {
    //     scrambled_word = String::new();
    //     println!("Enter a scrambled word:");
    //     let b1 = std::io::stdin().read_line(&mut scrambled_word);
    //     match b1 {
    //         Ok(_) => {
    //             let mut is_a_letter = true;
    //             for c in scrambled_word.trim_end().chars() {
    //                 if !c.is_alphabetic() {
    //                     println!("Invalid input, make sure the scrambled word is alphabetic.");
    //                     is_a_letter = false;
    //                     break;
    //                 }
    //             }
    //             if is_a_letter {
    //                 break;
    //             }
    //         }
    //         Err(_) => println!("Invalid input, try again."),
    //     };
    // }
    // println!(
    //     "{:?}",
    //     find_valid_words(
    //         /*replace following string with scrambled word you wish to unscramble. */
    //         //"iverdfdsdafefrgsfcrervrs"
    //         scrambled_word.as_str().trim()
    //     )
    // );
    let mut possible_words =
        find_words_with_known_and_unknown(&['y'], &['s', 'a', 'l', 'e', 't', 'd', 'o', 'r', 'k']);
    possible_words.retain(|x| x.chars().nth(4).unwrap() != 'y');
    println!("{:?}", possible_words)
}

// fn find_valid_words(letters: &str) -> Vec<String> {
//     let mut avail = vec![0_usize; 26];
//     // store the frequency of each letter in the string.
//     for c in letters.chars() {
//         let index = c as usize - 'a' as usize;
//         avail[index] += 1;
//     }

//     // create a vector to store the result.
//     let mut result: Vec<String> = Vec::new();

//     // open the dictionary file and create an input buffer.
//     let filename = "src/words_alpha.txt";
//     let file = File::open(filename).unwrap();
//     let reader = BufReader::new(file);

//     // for each word in the dictionary..
//     for word in reader.lines() {
//         let mut count = vec![0; 26];
//         let mut ok = true;
//         let word = word.unwrap();
//         /*
//          * count the frequency of each character in the dictionary word.
//          * set ok to false and break if too many of
//          * a letter is found in the dictionary word.
//          */
//         for c in word.chars() {
//             let index = c as usize - 'a' as usize;
//             count[index] += 1;
//             if count[index] > avail[index] {
//                 ok = false;
//                 break;
//             }
//         }
//         // if word has the right amount of each letter, add to result vector.
//         if ok && (word.len() == letters.len()) {
//             result.push(word);
//         }
//     }
//     result
// }

// fn find_words_with_letter(letter: char, in_position: usize) -> Vec<String>  {

//     // create a vector to store the result.
//     let mut result: Vec<String> = Vec::new();

//     // open the dictionary file and create an input buffer.
//     let filename = "src/words_alpha.txt";
//     let file = File::open(filename).unwrap();
//     let reader = BufReader::new(file);

//     // for each word in the dictionary..
//     for word in reader.lines() {

//         let word = word.unwrap();
//         /*
//          * count the frequency of each character in the dictionary word.
//          * set ok to false and break if too many of
//          * a letter is found in the dictionary word.
//          */
//         let mut letter_at_4th_ix= ' ';

//         if word.chars().count() == 5 {
//             letter_at_4th_ix = word.chars().nth(in_position).unwrap();
//             if letter == letter_at_4th_ix {
//                 result.push(word);
//             }
//         }
//     }
//     result
// }

fn find_words_with_known_and_unknown(letters_in: &[char], letters_out: &[char]) -> Vec<String> {
    // create a vector to store the result.
    let mut result: Vec<String> = Vec::new();

    // open the dictionary file and create an input buffer.
    let filename = "src/words_alpha.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // for each word in the dictionary..
    // let mut word_is_valid = true;
    'outer: for word in reader.lines() {
        let word = word.unwrap();
        if word.chars().count() == 5 {
            for letter in letters_in {
                let mut l = String::new();
                l.push(*letter);
                if !word.contains(&l) {
                    //word_is_valid = false;
                    break 'outer;
                }
            }
            for letter in letters_out {
                if word.contains(*letter) {
                    // word_is_valid = false;
                    break 'outer;
                }
            }
            result.push(word);
        }

    }
    result
}
