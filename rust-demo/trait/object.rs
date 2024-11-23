trait Greeting {
    fn greeting(&self) -> String;
}

struct Cat;
impl Greeting for Cat {
    fn greeting(&self) ->String {
        "Meow...".to_string()
    }
}

struct Dog;
impl Greeting for Dog {
    fn greeting(&self) ->String {
        "Bark...".to_string()
    }
}

fn print_greeting_static<G: Greeting>(g: G) {
    println!("{}", g.greeting());
}

fn print_greeting_dynamic(g:Box<dyn Greeting>){
    println!("{}",g.greeting());
}


fn main() {
    print_greeting_static(Cat);
    print_greeting_static(Dog);
    
    print_greeting_dynamic(Box::new(Cat));
    print_greeting_dynamic(Box::new(Dog));
}
