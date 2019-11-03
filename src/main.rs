
use xkcd1017;

use std::io::prelude::*;


pub fn main() {
	let mut p = xkcd1017::Progress::new();
	
	for i in 0..=10000 {
		p.set_progress(1.0 - i as f64 / 10000.0);
		print!("\x1B[2K\r{:>6.2}%: {:>25} - {}", p.get_progress() * 100.0, p.title, p.info);
		std::io::stdout().flush().ok();
		std::thread::sleep(std::time::Duration::from_millis(10));
	}
}
