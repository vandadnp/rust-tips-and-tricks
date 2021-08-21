// Want to support my work 🤝? https://buymeacoffee.com/vandad

fn main() {
    let foo = "Foo Bar";
    println!("Foo is {}", foo);

    let foo = String::from(foo);
    println!("Foo is {}", foo);
    assert_eq!(foo, foo);

    let bar: u32 = 0x10;
    let bar: i32 = bar as i32 + 0x10;
    assert_eq!(bar, 0x20);
}

