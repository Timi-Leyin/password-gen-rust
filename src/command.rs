use crate::args;
use crate::constants;
use crate::utils;
pub fn command(args:&args::Args){
    let _level = args.level;
    let mut password = String::new();


    // Generate password
    let letters_index = utils::generate_rand(constants::LETTERS.len() as u16);
    let numbers_index = utils::generate_rand(constants::NUMBERS.len() as u16);
    let special_characters_index = utils::generate_rand(constants::SPECIAL_CHARACTERS.len() as u16);

    password.push_str(&constants::LETTERS[letters_index as usize]);
    password.push_str(&constants::NUMBERS[numbers_index as usize]);
    password.push_str(&constants::SPECIAL_CHARACTERS[special_characters_index as usize]);
    println!("Password: {}", password);
}