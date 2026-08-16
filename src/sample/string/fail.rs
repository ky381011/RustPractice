// &str は不変参照なので push_str できない
fn str_push() {
    let s: &str = "hello";
    s.push_str(" world"); // error[E0599]: no method named `push_str` found for `&str`
}

// move 後の String は使えない
fn moved_value() {
    let s1 = String::from("hello");
    let _s2 = s1;
    println!("{}", s1); // error[E0382]: use of moved value: `s1`
}

// + 演算子の左辺は String でなければならない
fn str_concat() {
    let a: &str = "hello";
    let b: &str = " world";
    let _c = a + b; // error[E0369]: cannot add `&str` to `&str`
}

fn main() {}
