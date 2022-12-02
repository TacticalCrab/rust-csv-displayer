pub fn fill(data: &mut String, letter: char, len: usize) {
    let length = len - data.len();

    for _ in 0..length {
        data.push(letter);
    }
}