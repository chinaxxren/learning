trait Base {
    fn base(&self) {
        println!("base...");
    }
}

trait AsBase {
    fn as_base(&self) -> &dyn Base;
}

// blanket implementation
impl<T: Base> AsBase for T {
    fn as_base(&self) -> &dyn Base {
        self
    }
}

trait Foo: AsBase {
    fn foo(&self) {
        println!("foo..");
    }
}

#[derive(Debug)]
struct MyStruct;

impl Foo for MyStruct {}
impl Base for MyStruct {}

fn main() {
    let s = MyStruct;
    let foo: &dyn Foo = &s;
    foo.foo();
    
    let base: &dyn Base = foo.as_base();
    base.base();
}