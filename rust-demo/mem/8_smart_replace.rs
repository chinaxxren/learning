use std::mem;

fn main() {
    // Vector清空优化
    let mut vec = vec![1, 2, 3];
    let old_vec = mem::take(&mut vec);
    // 比 vec.clear() 更高效
    println!("Old vec: {:?}", old_vec);
    
    let mut opt = Some(String::from("old"));

    // 检查 opt 是否为 Some，如果是，则更新值
    if let Some(ref mut value) = opt {
        *value = String::from("new");
    }
    // 打印更新后的值
    println!("{:?}", opt); // 输出: Some("new")

    // Option更新优化
    let mut opt = Some(String::from("old"));
    if let Some(s) = opt.as_mut() {
        let old = mem::replace(s, String::from("new"));
        println!("Replaced: {}", old);
        println!("Updated: {:?}", opt); // 输出: Some("new")
    }
}