// Want to support my work 🤝? https://buymeacoffee.com/vandad

#[derive(Debug)]
struct Cat<'a> {
    name: &'a str,
    breed: &'a str,
    age: u8,
}

const CAT1: Cat = Cat {
    name: "Kitty 1",
    breed: "Ragdoll",
    age: 2,
};

fn instead_of_this() {
    let cat2 = Cat {
        name: "Kitty 2",
        breed: "Ragdoll",
        age: 2,
    };
    println!("Cat 1 = {:?} and cat 2 = {:?}", CAT1, cat2);
}

fn do_this() {
    let cat2 = Cat {
        name: "Kitty 2",
        ..CAT1
    };
    println!("Cat 1 = {:?} and cat 2 = {:?}", CAT1, cat2);
}

fn main() {
    instead_of_this();
    do_this();
}
