use std::mem;

fn main() {
    // size_of 获取类型大小
    println!("Size of i32: {} bytes", mem::size_of::<i32>());
    println!("Size of Option<i32>: {} bytes", mem::size_of::<Option<i32>>());

    // align_of 获取类型对齐要求
    println!("Alignment of i32: {} bytes", mem::align_of::<i32>());

    // size_of_val 获取值的大小
    let array = [0u8; 4];
    println!("Size of array: {} bytes", mem::size_of_val(&array));
}