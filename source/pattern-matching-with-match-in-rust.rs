struct Person {
    age: u8,
}

fn describe_person(person: &Person) -> &str {
    match person.age {
        0..=12 => "Child",
        13..=17 => "Teenager",
        _ => "Adult",
    }
}

fn main() {
    let foo = Person { age: 10 };
    // prints Foo is a Child
    println!("Foo is a {}", describe_person(&foo));
    let bar = Person { age: 13 };
    // prints Bar is a Teenager
    println!("Bar is a {}", describe_person(&bar));
    let baz = Person { age: 40 };
    // prints Baz is an Adult
    println!("Baz is an {}", describe_person(&baz));
}
