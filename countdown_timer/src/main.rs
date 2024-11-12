use std::io;
use std::io::{stdout, Write};
use tokio::time;
use tokio::time::Duration;


#[derive(Debug)]
struct CountdownTimer {
    name: String,
    start: u64,
    time_unit: TimeUnit,
}


impl CountdownTimer {
    fn new(name: String, start: u64, time_unit: TimeUnit) -> Self {
        CountdownTimer {
            name,
            start,
            time_unit,
        }
    }

    async fn start(&mut self) {
        let mut task_interval = time::interval(Duration::from_secs(1));
        let seconds = self.to_seconds();
        for _i in (0..=seconds).rev() {
            task_interval.tick().await;
            print!("\r{}    ", _i); // \r moves the cursor back to the start
            stdout().flush().unwrap(); // flush to ensure the output updates immediately
        }
    }

    fn to_seconds(&self) -> u64 {
        match self.time_unit {
            TimeUnit::HOUR => self.start * 60 * 60,
            TimeUnit::MINUTE => self.start * 60,
            TimeUnit::SECOND => self.start
        }
    }
}

#[derive(Debug)]
enum TimeUnit {
    HOUR,
    MINUTE,
    SECOND,
}


#[tokio::main]
async fn main() {
    println!("Countdown Timer is starting....");
    println!("Please enter countdown name:");
    stdout().flush().unwrap(); // Ensure the prompt is printed immediately

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read name");

    println!("Please enter countdown start:");
    stdout().flush().unwrap(); // Ensure the prompt is printed immediately
    let mut start_input = String::new();
    io::stdin().read_line(&mut start_input).expect("Failed to read start");


    let start: u64 = match start_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid integer.");
            return;
        }
    };

    let mut timer = CountdownTimer::new(name, start, TimeUnit::SECOND);
    println!("timer: {:?}", timer);
    timer.start().await;
    println!("\nBip bip bippp...");
}
