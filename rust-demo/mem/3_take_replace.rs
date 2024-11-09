use std::mem;

// 链表节点替换示例
struct Node {
    next: Option<Box<Node>>,
    value: i32,
}

impl Node {
    fn take_next(&mut self) -> Option<Box<Node>> {
        mem::take(&mut self.next)
    }

    fn replace_value(&mut self, new_value: i32) -> i32 {
        mem::replace(&mut self.value, new_value)
    }
}

fn main() {
    // take 经常用于处理Option
    let mut opt = Some(42);
    let value = mem::take(&mut opt);
    assert_eq!(value, Some(42));
    assert_eq!(opt, None);

    let mut node = Node {
        next: Some(Box::new(Node {
            next: None,
            value: 42,
        })),
        value: 10,
    };

    let mut next = node.take_next();
    
    // if let Some(a) = next.take() {
    //     println!("a: {}", a.value);
    // } else {
    //     println!("next is None");
    // }

    let next = next.unwrap();
    assert_eq!(next.value, 42);
    assert_eq!(node.value, 10);

    // replace 经常用于处理可变引用
    let old_value = node.replace_value(20);
    assert_eq!(old_value, 10);
    assert_eq!(node.value, 20);
}
