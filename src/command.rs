use crate::args;
use crate::utils;
pub fn command(args:&args::Args){
    let _level = args.level;
    let mut password = String::new();

    utils::generate_password(args.max, &mut password);
    // Generate password
    println!("Your Generated Password is\n {}", password);
}