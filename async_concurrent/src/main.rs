use reqwest::Error;
use tokio::task;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // 定义要请求的 URL 列表
    let urls = vec![
        "https://jsonplaceholder.typicode.com/todos/1",
        "https://jsonplaceholder.typicode.com/todos/2",
        "https://jsonplaceholder.typicode.com/todos/3",
    ];

    // 创建任务通道
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    // 并发启动多个请求任务
    for url in urls {
        let tx = tx.clone();
        task::spawn(async move {
            let client = reqwest::Client::new();
            match client.get(url).send().await {
                Ok(response) => {
                    if let Ok(body) = response.text().await {
                        tx.send(format!("{}: {}", url, body)).await.unwrap();
                    }
                }
                Err(e) => eprintln!("请求 {} 失败: {}", url, e),
            }
        });
    }

    // 等待所有任务完成并收集结果
    drop(tx); // 关闭发送端以终止接收循环
    while let Some(result) = rx.recv().await {
        println!("{}", result);
    }

    Ok(())
}