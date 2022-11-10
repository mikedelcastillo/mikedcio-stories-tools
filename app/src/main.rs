use std::thread;

use telegram::run_telegram_bot;

use dotenv::dotenv;

fn main() {
    dotenv().ok();
    files::setup();
    match client::APIClient::test_connection() {
        Ok(_) => println!("Connection successful."),
        Err(err) => panic!("Could not connect to the API. {:?}", err),
    }

    let mut app_threads = vec![];

    app_threads.push(thread::spawn(|| run_telegram_bot()));

    for app_thread in app_threads {
        app_thread.join().expect("Thread failed.");
    }
}
