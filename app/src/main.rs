use telegram::run_telegram_bot;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut app_threads = vec![];

    let app_thread = tokio::spawn(async { run_telegram_bot().await });
    app_threads.push(app_thread);

    for app_thread in app_threads {
        app_thread.await.expect("Thread failed.");
    }
}
