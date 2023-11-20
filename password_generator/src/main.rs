use rand::{distributions::Alphanumeric, Rng};

fn generate_password(password_length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(password_length)
        .map(char::from)
        .collect()
}

fn main() {
    let generated_password = generate_password(24);

    println!("Generated password: {}", generated_password)
}
