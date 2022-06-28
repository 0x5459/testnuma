use std::{
    env::args,
    time::{Duration, Instant},
};

mod mmap;
mod numa;

fn main() {
    let arg: usize = args().nth(1).and_then(|x| x.parse().ok()).unwrap_or(1);

    let sector_size = 1024 * 1024 * 32;
    let now = Instant::now();

    if arg == 1 {
        let _s = mmap::allocate_layer(sector_size).unwrap();
    } else {
        let _s = numa::allocate_layer(sector_size);
    }
    std::thread::sleep(Duration::from_secs(5));
    println!("elapsed: {}", now.elapsed().as_millis());
}
