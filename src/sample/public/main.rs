#[path = "calc.rs"]
mod calc; // calc.rs をモジュールとして読み込む

fn main() {
    let a = 10;
    let b = 3;

    println!("{} + {} = {}", a, b, calc::add(a, b));
    println!("{} - {} = {}", a, b, calc::sub(a, b));
    println!("{} * {} = {}", a, b, calc::mul(a, b));
    println!("{} / {} = {:.2}", a, b, calc::div(a, b as f64));
}
