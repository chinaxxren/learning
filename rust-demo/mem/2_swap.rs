use std::mem;

fn main() {
    let mut x = 1;
    let mut y = 2;

    // swap 交换两个可变引用的值
    mem::swap(&mut x, &mut y);
    assert_eq!(x, 2);
    assert_eq!(y, 1);

    let mut v = vec![1, 2, 3];

    // replace 替换值并返回原值
    let old_value = mem::replace(&mut v[0], 42);
    assert_eq!(old_value, 1);
    assert_eq!(v[0], 42);
}