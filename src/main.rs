use std::{
    env::args,
    time::{Duration, Instant},
};

mod mmap;
mod numa;

fn main() {
    let arg: usize = args().nth(1).and_then(|x| x.parse().ok()).unwrap_or(1);

    let sector_size = 1024 * 1024 * 1024 * 32;
    let now = Instant::now();

    if arg == 1 {
        let mut m = mmap::allocate_layer(sector_size).unwrap();
        unsafe {
            std::ptr::write_bytes::<u8>(m.as_mut_ptr(), 0, sector_size);
        }
    } else {
        let mut m = numa::allocate_layer(sector_size);
        let s = m.as_mut();
        unsafe {
            std::ptr::write_bytes::<u8>(s.as_mut_ptr(), 0, s.len());
        }
    };

    println!("elapsed: {}", now.elapsed().as_millis());

    std::thread::sleep(Duration::from_secs(5));
}
