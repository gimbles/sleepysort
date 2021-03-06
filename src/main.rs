use std::{thread, time, env::args};
use tokio::task::spawn_blocking;

#[tokio::main]
async fn main() {
    let mut argv = args().collect::<Vec<String>>();
    argv.remove(0);

    let numbers = argv.iter().map(|x| x.parse::<u64>().expect("Invalid number provided"));

    for i in numbers {
        spawn_blocking(move || {
            let mils = time::Duration::from_millis(i * 10);
            thread::sleep(mils);
            println!("{}", i);
        });
    }
}
