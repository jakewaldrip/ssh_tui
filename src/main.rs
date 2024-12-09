use app_server::AppServer;

mod app;
mod app_server;
mod terminal_handle;
mod widget;

#[tokio::main]
async fn main() {
    let mut server = AppServer::new();
    println!("Server listening");
    server.run().await.expect("Failed running server");
}
