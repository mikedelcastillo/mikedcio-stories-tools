use std::{thread, time::Duration};

use client::{self, try_surf};
use telegram::{run_telegram_bot};

struct Progress<'a> {
    text: &'a str,
    step_current: u32,
    step_total: u32,
}

impl<'a> Progress<'a> {
    fn new(step_total: u32) -> Self {
        Self {
            text: "",
            step_current: 0,
            step_total,
        }
    }

    fn set_text(&mut self, text: &'a str) -> &Self {
        self.text = text;
        self
    }

    fn step(&mut self, text: &'a str) -> &Self {
        self.set_text(text);
        self.step_current += 1;
        self
    }

    fn get_percent(&self) -> f32 {
        (self.step_current as f32) / (self.step_total as f32)
    }
}

#[tokio::main]
async fn main() {
    let n = client::add(100, 200);
    let p = client::get_url("api");
    println!("Hello, world! {}, {}", n, p);

    
    run_telegram_bot().await;
}

fn long_process(prog_hand: Box<dyn Fn(&Progress) -> Result<(), ()>>) -> Result<(), ()> {
    let mut prog = Progress::new(3);

    prog_hand(&prog.set_text("Starting process..."))?;
    thread::sleep(Duration::from_millis(200));

    prog_hand(&prog.step("Processing..."))?;
    thread::sleep(Duration::from_millis(200));

    prog_hand(&prog.step("Long process..."))?;
    thread::sleep(Duration::from_millis(2000));

    prog_hand(&prog.step("Done..."))?;

    Ok(())
}
