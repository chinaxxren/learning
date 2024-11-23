
use std::mem::transmute;
use std::fmt::Debug;

fn main() {
    let v = vec![1, 2, 3, 4];
    let a: &Vec<u64> = &v;
    // 转为 trait object
    let b: &dyn Debug = &v;
    println!("a: {}", a as *const _ as usize);
    println!("b: {:?}", unsafe { transmute::<_, (usize, usize)>(b) });
}