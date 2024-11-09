use std::mem;

#[repr(C)]
struct Optimized {
    a: u8,
    b: u32,
    c: u8,
}

#[repr(C, packed)]
struct Packed {
    a: u8,
    b: u32,
    c: u8,
}


//  内存对齐优化
fn main() {
    println!("Size of Optimized: {}", mem::size_of::<Optimized>());
    println!("Size of Packed: {}", mem::size_of::<Packed>());
    println!("Alignment of Optimized: {}", mem::align_of::<Optimized>());
    println!("Alignment of Packed: {}", mem::align_of::<Packed>());
}