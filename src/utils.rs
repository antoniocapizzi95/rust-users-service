use rand::{thread_rng, Rng};

pub fn generate_random_string(length: usize) -> String {
    let mut rng = thread_rng();
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
    (0..length).map(|_| chars[rng.gen_range(0..chars.len())]).collect()
}