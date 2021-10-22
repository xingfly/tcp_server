// 导入需要用到的类库
use std::{
    // io类库
    io::{Read, Write},
    // Tcp监听读取
    net::{TcpListener, TcpStream},
    // 多线程类库
    thread,
};

// main函数入口
fn main() {
    // 创建一个TCP监听器，监听127.0.0.1:8888端口（unwrap-解包装，如果是error会自动调用panic!宏）
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    // incoming接受客户端连接，一个新连接进入会产生一个Result枚举<TcpStrem,Error>
    for stream in listener.incoming() {
        // 通过模式匹配处理错误
        match stream {
            // 匹配到OK时的处理逻辑
            Ok(stream) => {
                // 开一个新的线程，处理多个请求的情况
                thread::spawn(move || {
                    // 应答函数，处理请求
                    reply(stream);
                });
            }
            // 匹配到错误时的处理；逻辑
            Err(e) => {
                // 输出错误信息，终止程序运行。
                panic!("无法连接，错误信息:{}", e);
            }
        }
    }
    // 关闭TCP连接
    drop(listener);
}

/**
 * 应答函数
 * @param stream: TcpStream TCP输入流
 *
*/
fn reply(mut stream: TcpStream) {
    // 定义一个缓冲器(需要填充内容所以需要用mut修饰)
    let mut buffer = [0; 1024];
    stream.write("应答程序(输入bye退出)\n".as_bytes()).unwrap();
    // 循环读取输入的信息
    loop {
        // read读取输入并填充到缓冲器，并返回bytes长度
        let bytes_len = stream.read(&mut buffer).expect("读取失败，程序中断运行");
        // 长度为零退户循环
        if bytes_len == 0 {
            break;
        }
        // 将byte[]转换为&str类型
        let response = match std::str::from_utf8(&buffer[..bytes_len]) {
            // 成功-返回&str
            Ok(value) => value,
            // 失败
            Err(_) => {
                // 返回错误信息
                stream.write("字符流格式不是utf-8\n".as_bytes()).unwrap();
                // 继续监听逻辑
                continue;
            }
        };
        // 检查输入内容是否为bye
        if response.to_lowercase().contains("bye") {
            // 向用户告别
            stream.write("再见\n".as_bytes()).unwrap();
            // 退出监听逻辑
            break;
        }
        // 应答逻辑，回应用户输入
        stream
            .write(format!("回应:{}", response).as_bytes())
            .unwrap();
    }
}
