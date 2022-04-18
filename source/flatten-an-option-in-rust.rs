// Want to support my work 🤝? https://buymeacoffee.com/vandad

#![deny(clippy::all)]

fn main() {
    let name1: Option<Option<&str>> = Some(Some("Foo"));
    let name = name1.flatten();
    println!("Name is {:}", name.unwrap_or("Unknown"));
}
