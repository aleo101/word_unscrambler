//Programmer: Alexander Leones
//Date: 2021-12-14
// This program returns an exhaustive list of english words that can be formed
// using no-more and no-less than every letter in a provided string.
/////// Credits: ///////////
//find_valid_words alogorithm was first published by Pham Trung at:
// https://stackoverflow.com/a/25298960
// dictionary txt file can be found at https://github.com/dwyl/english-words

use std::array;
use std::collections::{HashMap,};
use std::fs::File;
use std::io::{BufRead, BufReader};
mod does_contain;
/*
How it will work:

Enter rows previous to your guess of the correct word.
    - enter rows as vector of 5 tuple pairs.
    - each tuple's second value will be either:
        - 0 for grey
        - 1 for yellow
        - 2 for green
with this input:
    - go through each dictionary word, finding only words that contain all green/yellow letters and which don't contain grey.
    - further filter out words based off correct and incorrect positioning.
        - for each of the 5 char ix's, exclude all words that either contain a letter that was yellow at that ix, and further exclude
          words that don't contain the letter that is green at that ix.

add the int from every tuple to dict of all leters to get if the word ever appears in the word as green or yellow.
*/

fn get_input() -> Vec<[(char, usize); 5]> {
    vec![
        [('s', 0), ('a', 0), ('l', 0), ('e', 1), ('t', 1)],
        [('p', 1), ('e', 1), ('t', 1), ('t', 0), ('y', 0)],
    ]
}

fn determine_letters(touples: &Vec<[(char, usize); 5]>) -> (Vec<char>, Vec<char>, HashMap<char, usize>) {
    let mut known_letters = HashMap::new();
    let mut letter_map:[HashMap<char, usize>; 5] = [HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(),];
    
    for arr in touples {
        for t in arr {
            if *known_letters.entry(t.0).or_insert(t.1) < t.1 {
                known_letters.insert(t.0, t.1);
            }
        }
    }

    let mut green_or_yello: Vec<char> = vec![];
    let mut grey: Vec<char> = vec![];
    for letters in &known_letters {
        if letters.1 > &0 {
            green_or_yello.push(*letters.0);
        } else {
            grey.push(*letters.0);
        }
    }
    (green_or_yello, grey, known_letters) // temporary return
}

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


    // let mut possible_words =
    //     find_words_with_known_and_unknown(&['p', 'e', 't'], &['s', 'a', 'l', 'y']);
    // possible_words.retain(|x| {
    //     !x.starts_with('p')
    //         && x.chars().nth(1).unwrap() != 'e'
    //         && x.chars().nth(2).unwrap() != 't'
    //         && x.chars().nth(3).unwrap() != 'e'
    //         && x.chars().nth(4).unwrap() != 't'
    // });
    // println!("{:?}", possible_words)
    let input = get_input();
    let vectors = determine_letters(&input);
    let mut possible_words = find_words_with_known_and_unknown(&vectors.0, &vectors.1);
    // possible_words.retain(|x| {
    //     !x.starts_with('p')
    //         && x.chars().nth(1).unwrap() != 'e'
    //         && x.chars().nth(2).unwrap() != 't'
    //         && x.chars().nth(3).unwrap() != 'e'
    //         && x.chars().nth(4).unwrap() != 't'
    // });

    possible_words.retain(|x| {
        for arr in &input {
            let mut ix = 0;
            for letter in arr {
                if letter.1 == 1 {
                    if x.chars().nth(ix).unwrap() == letter.0 {
                        return false;
                    }
                } else if letter.1 == 2 {
                    if x.chars().nth(ix).unwrap() != letter.0 {
                        return false;
                    }
                }
                ix+=1;
            }
        }
        true
    });
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
                if !word.contains(*letter) {
                    //word_is_valid = false;
                    continue 'outer;
                }
            }
            for letter in letters_out {
                if word.contains(*letter) {
                    // word_is_valid = false;
                    continue 'outer;
                }
            }
            result.push(word);
        }
    }
    result
}
