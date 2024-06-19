fn main() {
	fizzbuzz(100);
}

fn fizzbuzz(count: u8) {
	for i in 0..count {
		println!("{}",
		match i {
			i if (i % 3 == 0 && i % 5 == 0) => String::from("fizzbuzz!"),
			i if i % 3 == 0 => String::from("fizz!"),
			i if i % 5 == 0 => String::from("buzz!"),
			other => other.to_string()
		});
	}
}

fn render() {
    // Image
	let image_width = 256;
	let image_height = 256;
	
	// Render
	
	println!("P3");
	println!("{image_width} {image_height}");
	println!("255");
	
	for j in 0..image_height {
		for i in 0..image_width {
			let r = f64::from(i) / f64::from(image_width - 1);
			let g = f64::from(j) / f64::from(image_height - 1);
			let b = 0.0;
			
			let ir = (255.999 * r) as u32;
			let ig = (255.999 * g) as u32;
			let ib = (255.999 * b) as u32;
			
			println!("{ir} {ig} {ib}");
		}
	}
}
