fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    // 使用索引访问向量的元素
    let first_number = numbers[0]; //first_number为1
    assert_eq!(first_number, 1);
    // 使用get方法访问向量的元素，索引越界得到None。
    assert_eq!(numbers.get(5), None);
}
