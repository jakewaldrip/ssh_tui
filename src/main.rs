use app_server::AppServer;

mod app_server;
mod terminal_handle;
mod widget;

struct App {
    pub counter: usize,
}

impl App {
    pub fn new() -> App {
        Self { counter: 0 }
    }
}

#[tokio::main]
async fn main() {
    let mut server = AppServer::new();
    println!("Server listening");
    server.run().await.expect("Failed running server");
}
