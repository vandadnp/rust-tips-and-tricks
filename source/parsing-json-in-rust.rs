// Free Flutter Course 💙 https://linktr.ee/vandadnp
// Want to support my work 🤝? https://buymeacoffee.com/vandad

use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Person (name: {}, age: {})", self.name, self.age)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let persons = reqwest::Client::builder()
        .build()?
        .get("http://localhost:5500/apis/persons.json")
        .send()
        .await?
        .json::<Vec<Person>>()
        .await?;
    println!("Persons are {:?}", persons);
    Ok(())
}

/* Cargo.toml dependencies
[dependencies]
reqwest = { version = "0.11", features = ["blocking", "json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0.136", features = ["derive"] }
*/
  
