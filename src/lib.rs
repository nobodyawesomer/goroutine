use core::time;


pub fn go<F>(run: F)
where
	F: FnOnce()
{
	run();
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn lets_go() {
		go(|| {
			println!("This runs!");
		});
	}
}