use core::time;

use goroutine::go;

fn count(n: u32, place: &str) {
	for i in 0..n {
		println!("Count from {place} is {i}");
	}
}

fn main() {

	go(|| count(15, "Idaho"));

	go(|| count(12+3, "Maine"));

	go(|| {
		count(15, "Oregon");
	});
}