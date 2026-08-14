fn main() {
    // スタック：固定長バイト配列（コンパイル時にサイズが決まる）
    let stack_str: [u8; 5] = *b"hello";
    let stack_view = std::str::from_utf8(&stack_str).unwrap();
    println!("stack: {} (addr: {:p})", stack_view, &stack_str);

    // ヒープ：String（実行時に動的確保）
    let heap_str: String = String::from("hello");
    println!("heap:  {} (addr: {:p})", heap_str, heap_str.as_ptr());
}
