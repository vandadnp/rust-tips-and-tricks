// 🐦 Twitter                   https://twitter.com/vandadnp
// 🔵 LinkedIn                  https://linkedin.com/in/vandadnp
// 🎥 YouTube                   https://youtube.com/c/vandadnp
// 🤝 Want to support my work?  https://buymeacoffee.com/vandad

#![deny(clippy::all)]

fn main() {
    let name1 = Some("Foo");
    let name2 = Some("Bar");
    if let (Some(name1), Some(name2)) = (name1, name2) {
        println!(
            "Both name1 ({:?}) and name2 ({:?}) are present!",
            name1, name2
        );
    } else {
        println!("Either one or both values are null");
    }
}
