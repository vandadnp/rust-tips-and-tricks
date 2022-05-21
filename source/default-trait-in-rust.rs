// 🐦 Twitter                   https://twitter.com/vandadnp
// 🔵 LinkedIn                  https://linkedin.com/in/vandadnp
// 🎥 YouTube                   https://youtube.com/c/vandadnp
// 🤝 Want to support my work?  https://buymeacoffee.com/vandad

#![deny(clippy::all)]

struct Person<'a> {
    first_name: &'a str,
    last_name: &'a str,
}

impl<'a> Default for Person<'a> {
    fn default() -> Self {
        Person {
            first_name: "Foo",
            last_name: "Bar",
        }
    }
}

fn main() {
    let person = Person::default();
    // Default = Person, first name = Foo, last name = Bar
    println!("First name = {:}", person.first_name);
    println!("Last name = {:}", person.last_name);
}
