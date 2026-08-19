// ── インラインモジュール定義 ──────────────────────────────────
mod greetings {
    pub fn hello(name: &str) -> String {
        format!("Hello, {}!", name)
    }

    // ネストしたモジュール
    pub mod formal {
        pub fn hello(name: &str) -> String {
            format!("Good day, {}.", name)
        }
    }
}

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
