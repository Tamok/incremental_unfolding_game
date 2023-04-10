use std::time::Duration;
use std::thread::sleep;

fn main() {
    loop {
        // Game logic goes here

        sleep(Duration::from_millis(100)); // Adjust the duration to control the game loop's speed
    }
}