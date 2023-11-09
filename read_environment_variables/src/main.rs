use std::env;

fn main() {
let name = "USER";

    match env::var(name) {
        Ok(v) => println!("{}: {}", name, v),
        Err(e) => panic!("${} is not set ({})", name, e)
    }
}
