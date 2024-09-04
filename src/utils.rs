use rand::Rng;
pub fn generate_rand (length:u16)->u16{
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..length);
    random_index
}