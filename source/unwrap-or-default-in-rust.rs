// Want to support my work 🤝? https://buymeacoffee.com/vandad

#![deny(clippy::all)]

fn unwrap_or_default<E: Default, T>(
    a: Option<T>, 
    b: Option<T>, 
    func: fn(T, T) -> E
) -> E {
    a.and_then(|value1| b.map(|value2| func(value1, value2)))
        .unwrap_or_default()
}

fn main() {
    let x = Some(10);
    let y = Some(20);
    let z = unwrap_or_default(x, y, |x, y| x + y); // 30
    println!("Z is {:?}", z);
}
