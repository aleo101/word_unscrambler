//Programmer: Alexander Leones
//Date: 2021-12-14
// This program returns an exhaustive list of english words that can be formed
// using no-more and no-less than every letter in a provided string.
/////// Credits: ///////////
//find_valid_words alogorithm was first published by Pham Trung at:
// https://stackoverflow.com/a/25298960
// dictionary txt file can be found at https://github.com/dwyl/english-words


use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
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

fn main() {
    let input = get_input();
    let vectors = determine_letters(&input);
    let mut potential_words = find_words_with_known_and_unknown(&vectors.0, &vectors.1);

    potential_words.retain(|x| {
        for (i, letter) in x.chars().enumerate() {
            if let Some(j) = vectors.2[i].get(&letter) {
                if j < &2 {
                    return false;
                }
            } else {
                for v in &vectors.2[i] {
                    if v.1 == &2 {
                        return false;
                    }
                }
            }
        }
        true
    });

    println!("{:?}", potential_words);
}

fn get_input() -> Vec<[(char, usize); 5]> {
    let mut guesses = vec![];
    'mainloop: loop {
        println!("Enter a word. All lowers, no spaces. \n Enter last to finish.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess);
        guesses.push(guess);
        if guesses.last().unwrap().trim() == "last" {
            guesses.pop();
            break 'mainloop;
        }
    }

    let mut res: Vec<[(char, usize); 5]> = vec![];
    let mut arr = [(' ', 0_usize); 5];

    for word in guesses {
        let mut tup = (' ', 0);
        let mut j = 0;
        for (i, character) in word.chars().enumerate() {
            if i % 2 != 0 {
                tup.1 = character.to_digit(10).unwrap() as usize;
                print!("{}", tup.0);
                arr[j] = tup;
                j += 1;
            } else {
                tup.0 = character;
            }
        }
        println!();
        res.push(arr);
    }
    res

    // vec![
    //     [('s', 0), ('a', 1), ('l', 0), ('e', 0), ('t', 0)],
    //     [('a', 1), ('u', 0), ('d', 0), ('i', 0), ('o', 0)],
    //     [('g', 0), ('r', 2), ('a', 2), ('p', 0), ('y', 2)],
    // ]
}

fn determine_letters(
    touples: &[[(char, usize); 5]],
) -> (Vec<char>, Vec<char>, [HashMap<char, usize>; 5]) {
    let mut known_letters = HashMap::new();
    let mut letter_map: [HashMap<char, usize>; 5] = [
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    ];

    for arr in touples {
        for (i, t) in arr.iter().enumerate() {
            if *letter_map[i].entry(t.0).or_insert(t.1) < t.1 {
                letter_map[i].insert(t.0, t.1);
            }
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
    (green_or_yello, grey, letter_map)
}

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
