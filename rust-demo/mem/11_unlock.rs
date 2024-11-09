use std::boxed::Box;
use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;

// 无锁栈结构，用于在多线程环境中安全地共享和修改栈
struct LockFreeStack<T> {
    // 栈的头节点，使用原子指针确保线程安全
    head: AtomicPtr<Node<T>>,
}

// 栈节点结构
struct Node<T> {
    // 下一个节点的指针
    next: *mut Node<T>,
    // 节点存储的值
    value: T,
}

impl<T> LockFreeStack<T> {
    // 向栈中添加一个新元素
    fn push(&self, value: T) {
        // 创建一个新节点，初始时next指针为空
        let new_node = Box::new(Node {
            next: std::ptr::null_mut(),
            value,
        });
        
        // 将新节点的Box转换为原始指针
        let new_ptr = Box::into_raw(new_node);

        // 使用CAS（Compare And Swap）循环直到成功添加新节点
        loop {
            // 加载当前栈的头节点指针
            let head = self.head.load(Ordering::Relaxed);
            // 将新节点的next指针设置为当前的头节点
            unsafe {
                (*new_ptr).next = head;
            }

            // 尝试使用CAS更新头节点指针为新节点的指针
            if self
                .head
                .compare_exchange(head, new_ptr, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                // 如果更新成功，则退出循环
                break;
            }
        }
    }
}

fn main() {
    // 创建一个LockFreeStack实例，用于存储整数类型
    let stack = LockFreeStack::<i32> {
        head: AtomicPtr::new(std::ptr::null_mut()),
    };

    let stack_ar = Arc::new(stack);
    // 在多个线程中同时调用push方法，向栈中添加元素
    let handles: Vec<_> = (0..10)
        .map( |i| {
            let stack = stack_ar.clone();
            std::thread::spawn({
                move || {
                    stack.push(i);
                }
            })
        })
        .collect();

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }    
    println!("Thread finished");

    unsafe {
        drop(Box::from_raw(stack_ar.clone().head.load(Ordering::Relaxed)));
    }
    println!("Stack head freed");
}
