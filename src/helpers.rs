pub fn vec_of_strings(input: &str) -> Vec<&str> {
    input.split('\n').map(|str| str.trim()).collect()
}

pub fn vec_of_numbers(input: &str) -> Vec<u32> {
    let strings = vec_of_strings(input);
    strings.iter().map(|str| str.parse().unwrap()).collect()
}

pub fn get_alphabet() -> Vec<String> {
    (b'a'..=b'z')
        .chain(b'A'..=b'Z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
}
