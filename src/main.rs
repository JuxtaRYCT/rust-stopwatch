use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let mut stdout = stdout();
    let start = Instant::now();

    loop {
        let elapsed = start.elapsed();
        let seconds = elapsed.as_secs() % 60;
        let minutes = (elapsed.as_secs() / 60) % 60;
        let hours = elapsed.as_secs() / 3600;

        print!("\x1B[2J\x1B[1;1H");
        println!("{}", create_ascii_clock(hours, minutes, seconds));
        stdout.flush().unwrap();

        sleep(Duration::from_millis(100));
    }
}

fn create_ascii_clock(hours: u64, minutes: u64, seconds: u64) -> String {
    let digits = [
        [
            " __ ", "   ", " __ ", " __ ", "    ", " __ ", " __ ", " __ ", " __ ", " __ ",
        ],
        [
            "|  |", "  |", " __|", " __|", "|__|", "|__ ", "|__ ", "  __|", "|__|", "|__|",
        ],
        [
            "|__|", "  |", "|__ ", " __|", "   |", " __|", "|__|", "   |", "|__|", " __|",
        ],
    ];

    let mut clock = String::new();
    let time = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);

    for row in 0..3 {
        for ch in time.chars() {
            if ch == ':' {
                clock.push_str(if row == 1 { " . " } else { "   " });
            } else {
                let digit = ch.to_digit(10).unwrap() as usize;
                clock.push_str(digits[row][digit]);
            }
        }
        clock.push('\n');
    }

    clock
}
