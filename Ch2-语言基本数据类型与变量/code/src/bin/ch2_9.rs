fn main() {
    let INDEX: usize = 10;
    let mut numbers = [1, 2, 3, 4, 5];
    // numbers[INDEX] = 10;  // 导致编译错误

    let index_read = std::env::args().nth(1).unwrap().parse::<usize>().unwrap();
    let value = numbers[index_read]; //index_read > 4将导致运行时错误
}
