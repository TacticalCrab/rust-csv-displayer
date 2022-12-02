pub fn fill(data: &mut String, letter: char, len: usize) {
    let length = len - data.chars().count();

    for _ in 0..length {
        data.push(letter);
    }
}