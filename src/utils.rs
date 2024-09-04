use crate::constants;
use rand::Rng;
pub fn generate_rand(length: u16) -> u16 {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..length);
    random_index
}

pub fn generate_password(length: u16, s: &mut String) {
    let _special_characters_index = generate_rand(constants::SPECIAL_CHARACTERS.len() as u16);
    for _ in 0..length {
        let letters_index = generate_rand(constants::LETTERS.len() as u16);
        let numbers_index = generate_rand(constants::NUMBERS.len() as u16);

        s.push_str(&constants::LETTERS[letters_index as usize]);
        s.push_str(&constants::NUMBERS[numbers_index as usize]);
    }
    // s.push_str(&constants::SPECIAL_CHARACTERS[special_characters_index as usize]);
}
