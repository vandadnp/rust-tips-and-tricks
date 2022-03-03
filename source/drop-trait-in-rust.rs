// Free Flutter Course 💙 https://linktr.ee/vandadnp
// Want to support my work 🤝? https://buymeacoffee.com/vandad

use std::fmt;

struct Sheep {}

impl Drop for Sheep {
    fn drop(&mut self) {
        println!("Sheep is dropped 😢")
    }
}

impl fmt::Display for Sheep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "🐑")
    }
}

fn main() {
    let sheep = Sheep {};
    println!("Sheep is {}", sheep);
}
