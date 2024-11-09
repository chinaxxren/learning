
use std::mem;

fn main() {
    // 危险但有用的类型转换
    unsafe {
        // 将i32转换为[u8; 4]
        let num: i32 = 42;
        let bytes: [u8; 4] = mem::transmute(num);
        println!("bytes: {:?}", bytes);

        
        // 将裸指针转换为usize
        let ptr: *const i32 = &42;
        let addr: usize = mem::transmute(ptr);
        println!("addr: {:#x}", addr);


        // 将生命周期标记去除（极其危险！）
        let string = "hello";
        let static_str: &'static str = mem::transmute(string);
        println!("static_str: {}", static_str);
    }
}