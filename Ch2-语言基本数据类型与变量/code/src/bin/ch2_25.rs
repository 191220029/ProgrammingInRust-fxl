union Shape {
    circle: f64,
    rectangle: (f64, f64),
}

fn main() {
    let mut circ = Shape { circle: 1.0 };
    // 读取 union 的字段总是不安全的
    assert_eq!(unsafe { circ.circle }, 1.0);
    // 通过任何字段进行更新都会修改所有字段
    circ.rectangle = (2.0, 2.0);
    assert_eq!(unsafe { circ.circle }, 2.0);
}
