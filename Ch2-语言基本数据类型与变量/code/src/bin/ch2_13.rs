fn main() {
    // 创建一个元组
    let person = ("Alice", 30);

    // 使用模式匹配来解构元组
    let (name, age) = person;

    assert_eq!(name, "Alice");
    assert_eq!(age, 30);
}
