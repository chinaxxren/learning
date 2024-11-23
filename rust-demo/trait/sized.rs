trait Foo {
    fn method1(&self);
    fn method2(&mut self, x: i32, y: String) -> usize;
}

struct MyObject;

impl Foo for MyObject {
    fn method1(&self) {
        println!("MyObject method1");
    }

    fn method2(&mut self, x: i32, y: String) -> usize {
        println!("MyObject method2: {}, {}", x, y);
        x as usize
    }
}

fn main() {
    let obj = MyObject;
    let mut obj_box: Box<dyn Foo> = Box::new(obj); // 使用 Box 来拥有所有权

    obj_box.method1();
    let result = obj_box.method2(42, "Hello".to_string());
    println!("Result: {}", result);
}