use std::mem;

fn main() {
    // 使用size_of优化泛型代码
    fn is_zero_sized<T>() -> bool {
        mem::size_of::<T>() == 0
    }

    assert!(is_zero_sized::<()>());
    assert!(!is_zero_sized::<i32>());

    // 编译时计算大小
    let union = mem::size_of::<()>();
    assert_eq!(union, 0);
    
    const SIZE_OF_I32: usize = mem::size_of::<i32>();
    assert_eq!(SIZE_OF_I32, 4);
}