use std::collections::HashMap;
use std::io::ErrorKind;
use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, Mutex};

#[derive(Default)]
struct ChatServer {
    rooms: Mutex<HashMap<String, broadcast::Sender<String>>>,
}

impl ChatServer {
    // 异步函数，用于处理客户端加入聊天房间的逻辑
    async fn join_room(&self, room_name: &str, user_name: &str, socket: TcpStream) {
        // 声明一个可变变量 rx，用于接收房间的广播消息
        let mut rx;
        {
            // 获取互斥锁，以访问和修改 rooms 哈希表
            let mut rooms = self.rooms.lock().await;
            // 在 rooms 哈希表中查找指定房间名的发送器，如果不存在则创建一个新的
            let room_tx = rooms.entry(room_name.to_string()).or_insert_with(|| {
                // 创建一个广播通道，容量为 10
                let (tx, _) = broadcast::channel(10);
                // 打印创建房间的信息
                println!("Creating room: {}", room_name);
                // 返回新创建的发送器
                tx
            });
            // 格式化一条消息，通知其他用户有新用户加入房间
            let msg = format!("{} joined the room.", user_name);
            // 尝试将消息发送到房间的发送器，如果发送失败则忽略错误
            let _ = room_tx.send(msg);
            // 订阅房间的广播消息，并将接收器赋值给 rx
            rx = room_tx.subscribe();
        }

        // 将 TcpStream 封装在 Arc 和 Mutex 中，以便在多个任务间安全共享和访问
        let socket = Arc::new(Mutex::new(socket));
        // 克隆 Arc，以便在另一个任务中使用
        let socket_clone = Arc::clone(&socket);

        // 启动一个新的异步任务，用于接收房间的广播消息并发送给用户
        tokio::spawn(async move {
            // 进入一个无限循环，持续接收消息
            loop {
                // 尝试从 rx 接收器中接收消息
                match rx.recv().await {
                    // 如果成功接收到消息
                    Ok(msg) => {
                        // 获取 socket 的互斥锁
                        let mut socket = socket_clone.lock().await;
                        // 将消息写入到用户的 socket 中，如果失败则打印错误信息
                        if let Err(e) = socket.write_all(format!("{}\n", msg).as_bytes()).await {
                            println!("Error sending message: {}", e);
                            // 发生错误，退出循环
                            break;
                        }
                    }
                    // 如果接收消息时发生滞后错误
                    Err(broadcast::error::RecvError::Lagged(lag)) => {
                        // 打印滞后的消息数量
                        println!("Lagged behind on {} messages", lag);
                    }
                    // 如果接收消息时通道已关闭
                    Err(_) => {
                        // 打印通道关闭的信息
                        println!("Channel closed");
                        // 退出循环
                        break;
                    }
                }
            }
        });

        // 进入一个无限循环，处理从用户 socket 读取的数据
        loop {
            // 创建一个 1024 字节的缓冲区
            let mut buf = [0; 1024];
            // 获取 socket 的互斥锁
            let mut socket = socket.lock().await;
            // 尝试从 socket 中读取数据
            match socket.read(&mut buf).await {
                // 如果读取到 0 字节，表示连接已关闭
                Ok(n) if n == 0 => break,
                // 如果读取成功
                Ok(n) => {
                    // 格式化读取到的消息，包含用户名和消息内容
                    let msg = format!("{}: {}", user_name, String::from_utf8_lossy(&buf[..n]));
                    // 尝试将消息发送到房间的发送器，如果发送失败则打印错误信息并退出循环
                    if let Err(_) = self.rooms.lock().await.get(room_name).unwrap().send(msg) {
                        println!("Error sending message to room");
                        break;
                    }
                }
                // 如果读取操作会阻塞
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    // 忽略该错误，继续循环
                    continue;
                }
                // 如果发生其他错误
                Err(e) => {
                    // 打印错误信息并退出循环
                    println!("Error reading from socket: {:?}", e);
                    break;
                }
            }
        }
        // 调用 leave_room 函数，通知其他用户该用户已离开房间
        self.leave_room(room_name, user_name).await;
    }

    /// 异步离开房间的方法
    ///
    /// 该方法用于让用户离开一个指定的房间它首先尝试获取房间列表的锁，
    /// 然后检查指定名称的房间是否存在如果房间存在，它会构建一条消息，
    /// 表示用户离开了房间，并通过房间的发送器（tx）发送这条消息
    ///
    /// # 参数
    ///
    /// * `room_name`: &str - 房间的名称，用于标识用户想要离开的房间
    /// * `user_name`: &str - 用户的名称，用于构建离开消息
    async fn leave_room(&self, room_name: &str, user_name: &str) {
        // 获取房间列表的锁，以安全地访问共享的房间数据
        let rooms = self.rooms.lock().await;

        // 检查是否存在指定名称的房间如果存在，获取其发送器
        if let Some(tx) = rooms.get(room_name) {
            // 构建用户离开房间的消息
            let msg = format!("{} left the room.", user_name);

            // 发送消息，这里忽略了发送结果，因为无论如何处理结果都不影响离开房间的操作
            let _ = tx.send(msg);
        }
        // 如果房间不存在，该方法什么也不做
    }
}

#[tokio::main]
async fn main() {
    // 绑定到本地地址 127.0.0.1:8080，并等待连接
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    // 打印服务器正在监听的地址
    println!("Server listening on {}", listener.local_addr().unwrap());

    // 创建一个默认的聊天服务器实例，并将其包装在 Arc 中，以便在多个任务之间安全共享
    let chat_server = Arc::new(ChatServer::default());

    // 进入一个无限循环，接受客户端连接并处理它们
    loop {
        // 接受一个新的 TCP 连接，并等待结果
        let (mut socket, addr) = listener.accept().await.unwrap();
        // 打印新客户端连接的地址
        println!("New client connected: {}", addr);

        // 克隆聊天服务器的 Arc 实例，以便在新任务中使用
        let chat_server = chat_server.clone();
        // 在 Tokio 中生成一个新的异步任务来处理客户端连接
        tokio::spawn(async move {
            // 创建一个 1024 字节的缓冲区，用于读取客户端发送的数据
            let mut buf = [0; 1024];
            // 尝试从客户端套接字读取数据，并等待结果
            if let Ok(n) = socket.read(&mut buf).await {
                // 将读取的字节转换为 UTF-8 字符串
                let msg = String::from_utf8_lossy(&buf[..n]);
                // 将消息分割成房间名和用户名两部分
                let parts: Vec<&str> = msg.splitn(2, ':').collect();
                // 如果消息格式正确（包含房间名和用户名）
                if parts.len() == 2 {
                    // 提取房间名和用户名
                    let room_name = parts[0].trim();
                    let user_name = parts[1].trim();
                    // 让客户端加入指定的房间
                    chat_server.join_room(room_name, user_name, socket).await;
                } else {
                    // 如果消息格式不正确，打印错误信息
                    println!("Invalid message format from {}: {}", addr, msg);
                }
            }
        });
    }
}
