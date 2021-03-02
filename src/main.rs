mod app;

#[tokio::main]
async fn main() {
    app::run(18001).await;
}

