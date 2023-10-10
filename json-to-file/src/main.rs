use serde::{Serialize, Deserialize};
use serde_json::{Result};

#[derive(Serialize, Deserialize)]
struct Animal {
    name: String,
    age: u8,
    species: String,
}

fn main() -> Result<()> {
    let a = Animal {
        name: String::from("Pluto"),
        age: 2,
        species: String::from("Dog"),
    };

    // Write to file
    let mut file = std::fs::File::create("animal.json").expect("");
    serde_json::to_writer_pretty(&mut file, &a)?;

    // Print to console
    let json = serde_json::to_string_pretty(&a)?;
    println!("{} \nWas written to file", json);

    Ok(())
}