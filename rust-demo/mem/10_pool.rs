use std::mem;

struct Pool<T> {
    items: Vec<T>,
}

impl<T: Default> Pool<T> {
    fn get(&mut self) -> T {
        if self.items.is_empty() {
            T::default()
        } else {
            // 使用take避免额外的内存分配
            mem::take(&mut self.items[0])
        }
    }

    fn return_item(&mut self, item: T) {
        self.items.push(item);
    }
}

fn main() {
    let mut pool = Pool { items: vec![1,2,3] };
    let item1 = pool.get();
    println!("item1: {}", item1);
    let item2 = pool.get();
    println!("item2: {}", item2);
    let item3 = pool.get();
    println!("item3: {}", item3);
    println!("{:?}", pool.items);
    
    pool.return_item(item1);
    pool.return_item(item2);
    pool.return_item(item3);
    println!("{:?}", pool.items);
}
