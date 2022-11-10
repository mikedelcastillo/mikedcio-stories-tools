use std::thread;

use telegram::run_telegram_bot;

use dotenv::dotenv;

fn main() {
    dotenv().ok();
    files::setup();

    let mut app_threads = vec![];

    app_threads.push(thread::spawn(|| run_telegram_bot()));

    for app_thread in app_threads {
        app_thread.join().expect("Thread failed.");
    }
}
