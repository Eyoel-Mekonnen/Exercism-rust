pub fn reverse(input: &str) -> String {
    //todo!("Write a function to reverse {input}");
    let mut new_string = String::new();
    for character in input.chars().rev() {
        new_string.push(character)
    }
    return new_string;
}
