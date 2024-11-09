use std::mem;

 // forget 防止析构函数被调用
 struct NeedCleanup {
    data: Vec<u8>,
}

impl Drop for NeedCleanup {
    fn drop(&mut self) {
        println!("Cleaning up!");
    }
}

fn main() {

    let cleanup = NeedCleanup {
        data: vec![1, 2, 3],
    };

    // 使用forget后不会调用drop,所以就不会打印Cleaning up!
    mem::forget(cleanup);

    // 注意：这可能导致内存泄漏，慎用！
    // drop(cleanup);
}