use std::collections::HashSet;

fn does_contain(word: &mut String, letters: &[char]) -> bool {
    let letters_set: HashSet<&char> = HashSet::from_iter(letters);
    // let original_size = word.chars().count();
    // word.retain(|x| !letters_set.contains(&x));
    // if word.chars().count() == (original_size -  letters_set.len()) {
    //     return true
    // }
    // false
    for l in letters_set {
        if !word.contains(*l) {
            return false;
        }
    }
    true
}
