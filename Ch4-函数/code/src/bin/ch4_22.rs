fn main() {
    let x = 1;
    let i = 10;

    let incr = |x| x + i;

    assert_eq!(incr(x), 11);
    incr(x); //可以重复调用
}
