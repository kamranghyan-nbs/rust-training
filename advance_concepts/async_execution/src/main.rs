#![allow(dead_code, unused_variables)]

#[tokio::main]

async fn main() {
    let fut = foo().await;
}

async fn foo() -> u32 {
    println!("In foo!");
    5
}