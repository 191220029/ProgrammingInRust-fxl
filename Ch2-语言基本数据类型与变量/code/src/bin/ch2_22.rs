use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::new();

    // 添加元素
    list.push_back(1);
    list.push_front(0);

    // 取出元素
    assert_eq!(list.pop_front(), Some(0)); // 取出头部元素
    assert_eq!(list.pop_back(), Some(1)); // 取出尾部元素
    assert_eq!(list.pop_back(), None); // 取出尾部元素
}
