use postgres::{Client, NoTls};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::connect("host=localhost user=postgres password=mysecretpassword", NoTls)?;

    let db_name = "test_database";
    let username = "test_user";
    let password = "testPassword";

    client.execute(format!("CREATE DATABASE {}", db_name).as_str(), &[])?;

    client.execute(format!("CREATE USER {} WITH PASSWORD '{}'", username, password).as_str(), &[])?;

    Ok(())
}
