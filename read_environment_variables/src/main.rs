use std::env;

fn main() {
    let name: &'static str = "USER";

    match env::var(name) {
        Ok(v) => println!("{}: {}", name, v),
        Err(e) => panic!("${} is not set ({})", name, e)
    }

    let user_env_variable = option_env!("USER").unwrap();
    println!("{:?} is the current user", user_env_variable);
}
