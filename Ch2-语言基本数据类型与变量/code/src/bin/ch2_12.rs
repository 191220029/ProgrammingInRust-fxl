fn main() {
    // 创建一个元组
    let person = ("Alice", 30);

    // 访问元组的元素
    let name = person.0;
    let age = person.1;

    assert_eq!(name, "Alice");
    assert_eq!(age, 30);
}
