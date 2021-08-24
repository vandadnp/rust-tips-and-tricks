// Want to support my work 🤝? https://buymeacoffee.com/vandad

#[derive(Debug)]
struct Dog {
    name: String,
    race: String,
    age: u32,
}

impl PartialEq for Dog {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.race == other.race && self.age == other.age
    }
}

impl Eq for Dog {}

fn instead_of_this(name: String, race: String) -> Dog {
    Dog {
        name: name,
        race: race,
        age: 1,
    }
}

fn do_this(name: String, race: String) -> Dog {
    Dog {
        name: name,
        race,
        age: 1,
    }
}

fn main() {
    let dog1 = instead_of_this(String::from("Molly"), String::from("Labradoodle"));
    let dog2 = do_this(String::from("Molly"), String::from("Labradoodle"));
    assert_eq!(dog1, dog2);
}
