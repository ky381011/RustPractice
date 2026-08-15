fn main() {
    // スタック：固定長バイト配列（コンパイル時にサイズが決まる）
    let stack_str: [u8; 5] = *b"hello";
    let stack_view = std::str::from_utf8(&stack_str).unwrap();
    println!("stack: {} (addr: {:p})", stack_view, &stack_str);

    // ヒープ：String（実行時に動的確保）
    let heap_str: String = String::from("hello");
    println!("heap:  {} (addr: {:p})", heap_str, heap_str.as_ptr());

    // with_capacity：再アロケーションを避けるため容量を事前確保
    let mut s = String::with_capacity(16);
    println!("capacity before push: {}", s.capacity());
    s.push_str("hello");
    s.push(' ');
    s.push_str("world");
    println!("with_capacity: {} (len: {}, capacity: {})", s, s.len(), s.capacity());
}
