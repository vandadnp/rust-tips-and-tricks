// Free Flutter Course 💙 https://linktr.ee/vandadnp
// Want to support my work 🤝? https://buymeacoffee.com/vandad

#[macro_use]
extern crate maplit;

#[derive(Debug)]
enum Value<'a> {
    INTEGER(i32),
    STRING(&'a str),
}

fn main() {
    let map = hashmap! {
        "name" => Value::STRING("Foo bar"),
        "age" => Value::INTEGER(20),
        "country" => Value::STRING("Sweden")
    };
    let values: Vec<Value> = map
        .into_values()
        .filter(|v| match v {
            Value::INTEGER(_) => true,
            _ => false,
        })
        .collect();
    println!("{:?}", values);
}
