use std::{thread, time};
use tokio::task::spawn_blocking;

#[tokio::main]
async fn main() {
    let unsorted = [6, 7, 3, 5, 1, 7, 9];

    for i in unsorted {
        spawn_blocking(move || {
            let mils = time::Duration::from_millis(i * 10);
            thread::sleep(mils);
            println!("{}", i);
        });
    }
}
