fn main() {
    // パターン1: T — 所有権あり・変更不可
    let s = String::from("hello");
    println!("[T]     s = {}", s);

    // パターン2: mut T — 所有権あり・変更可
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("[mut T] s = {}", s);

    // パターン3: &T — 不変参照（借用）・変更不可・複数同時に持てる
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s; // 複数の不変参照は同時に持てる
    println!("[&T]    r1 = {}, r2 = {}", r1, r2);

    // パターン4: &mut T — 可変参照（可変借用）・変更可・同時に1つだけ
    let mut s = String::from("hello");
    let r = &mut s;
    r.push_str(", world");
    println!("[&mut T] r = {}", r);
}
