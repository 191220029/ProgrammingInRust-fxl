fn main() {
    let mut text = String::from("Hello, Rust programmers!");
    // 添加内容
    text.push_str(" Welcome to our class.");
    assert_eq!(text, "Hello, Rust programmers! Welcome to our class.");
    // 移除最后一个字符
    assert_eq!(text.pop(), Some('.'));
    assert_eq!(text, "Hello, Rust programmers! Welcome to our class");
    // 替换内容
    text = text.replace("class", "Rust workshop.");
    assert_eq!(
        text,
        "Hello, Rust programmers! Welcome to our Rust workshop."
    );
    // 拼接字符串
    text = format!("{}{}", text, " Let's start.");
    assert_eq!(
        text,
        "Hello, Rust programmers! Welcome to our Rust workshop. Let's start."
    );
    // 清空字符串
    text.clear();
    assert_eq!(text, "");
}
