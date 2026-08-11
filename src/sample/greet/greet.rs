fn greet() -> String {
    let greeting = format!("Hello, {}!", "Alice");
    // println!("{}", greeting);
    greeting // ; をつけないことで返り値となる
}

fn main() {
    let greeting = greet();
    println!("{}!!", greeting);
}