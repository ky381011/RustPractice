// greetings.rs を同じディレクトリから読み込む
mod greetings;

// use でパスを短縮
use greetings::formal;

fn main() {
    // フルパスでアクセス
    println!("{}", greetings::hello("Alice"));

    // use で短縮したパス
    println!("{}", formal::hello("Bob"));

    // ブロックスコープ内の use
    {
        use greetings::hello;
        println!("{}", hello("Carol"));
    }
}
