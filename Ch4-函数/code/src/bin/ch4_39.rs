fn main() {
    let v = vec![1, 2, 3];
    let mut verify = vec![4, 3];

    let v_plus = v.iter().map(|x| *x + 1);
    let u = v_plus.filter(|x| *x > 2);
    u.for_each(|x| assert_eq!(Some(x), verify.pop()));
}
