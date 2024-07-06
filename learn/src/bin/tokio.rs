use std::{future::Future, task::{Poll, Context}};
use tokio::time::{sleep, Duration};
use tokio::select;

// select!宏的基础用法
// select! 允许在多个异步计算中等待，并在单个计算完成后返回。
// Future 是 Tokio 中的一个重要概念。它代表了一个异步操作的未来结果
// Future 是一个 trait ，它定义了一部操作的执行和返回值
// 我们可以通过实现 Future trait 来定义自己的异步操作

struct MyFuture;

impl Future for MyFuture {
    // 指定输出类型
    type Output = String;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Poll::Pending / Poll::Ready()
        Poll::Ready(String::from("Hello world!"))
    }

}  

// Task 代表 tokio 异步操作的执行上下文
// 每个 Task 都有一个关联的 Future，Task 负责执行Future 的异步操作，并在完成时返回结果（就是调用 poll？）

// 每个 Task 都由 Executor 来管理
// Executor 是一个可以执行异步才操作的线程池，它负责调度和执行所有的异步操作。
// 当我们创建一个 Task 时，它会被分配到 Executor 中的一个线程上，并在该线程上执行异步操作。

// select! 宏的基本语法

/// ```
/// select! {
///     patten1 => { // 一个异步代码块（返回 Future） }
///     patten2 => { // 一个异步代码块（返回 Future） }
/// }
/// ```

struct SelectDoc;



async fn future1() -> String {
    sleep(Duration::from_secs(1)).await;
    String::from("future1")
}

async fn future2() -> String {
    sleep(Duration::from_secs(2)).await;
    String::from("future2")
}

#[tokio::main]
async fn main() {
    

    select! {
        result1 = future1().fuse() => {
            println!("future1 completed with result: {}", result1);
        }

        result2 = future2().fuse() => {
            println!("future2 completed with result: {}", result2);
        }
    }



    println!("tokio {}", 1);
 }