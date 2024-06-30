use rustracer::vector::Vec3;

#[derive(Copy,Clone)]
enum Color {
	White,
	Red,
	Green,
	Blue
}

impl Color {
	fn toRGB(&self) -> Vec3<u8> {
		match *self {
			Color::White => Vec3::new(255,255,255),
			Color::Red => Vec3::new(255,0,0),
			Color::Green => Vec3::new(0,255,0),
			Color::Blue => Vec3::new(0,0,255),
		}
	}
}



struct Sphere {
	radius: f64,
	origin: Vec3<f64>,
	color: Color
}

fn main() {
		
	// scene
	
	let objects = vec![
		Sphere {
			origin: Vec3::new(0., -1., 3.),
			radius: 1.,
			color: Color::Red
		},
		Sphere {
			origin: Vec3::new(2., 0., 4.),
			radius: 1.,
			color: Color::Blue
		},
		Sphere {
			origin: Vec3::new(-2., 0., 4.),
			radius: 1.,
			color: Color::Green
		},
	];
	
	let viewport_width = 1.;
	let viewport_height = 1.;
	let viewport_distance = 1.;
	let origin = Vec3 { x: 0., y: 0., z: 0. };
	
	//draw
	let canvas_width = 600;
	let canvas_height = 400;
	
	let infinity = i32::MAX;
	
	let (c_w_start, c_w_end, c_h_start, c_h_end) = get_canvas_boundaries(canvas_width, canvas_height);
	
	println!("P3");
	println!("{canvas_width} {canvas_height}");
	println!("255");
	
	for x in c_w_start..c_w_end {
		for y in c_h_start..c_h_end {
			let direction = canvas_to_viewport(x,y,viewport_width,viewport_height,viewport_distance,canvas_width,canvas_height);
			let color = trace_ray(&origin, &direction, 1, infinity, &objects[..]);
			
			print_color(&color);
		}
	}
}

fn write_color(color: Vec3<i32>) {
	println!("{0} {1} {2}", color.x, color.y, color.z);
}

fn print_color(color: &Color) {
	let color = color.toRGB();
	println!("{0} {1} {2}", color.x, color.y, color.z);
}

// make enumerator?
fn get_canvas_boundaries(width: i32, height: i32) -> (i32, i32, i32, i32) {
	let canvas_width_end = width / 2;
	let canvas_width_start = canvas_width_end * -1;
	let canvas_height_end = height / 2;
	let canvas_height_start = canvas_height_end / 2;
	
	return (canvas_width_start, canvas_width_end, canvas_height_start, canvas_height_end);
}

fn canvas_to_viewport(x: i32, y: i32, vw: f64, vh: f64, vd: f64, cw: i32, ch: i32) -> Vec3<f64> {
	
	let width_scalar = vw / cw as f64;
	let height_scalar = vh / cw as f64;
	
	return Vec3 {
		x: x as f64 * width_scalar,
		y: y as f64 * height_scalar,
		z: vd
	};
}

fn is_closest_intersection(t: f64, start: i32, end: i32, closest_intersection: f64) -> bool {
	t >= start.into() &&
	t <= end.into() &&
	t < closest_intersection
}

// Spheres will eventually be all traceable objects
fn trace_ray(o: &Vec3<f64>, d: &Vec3<f64>, start: i32, end: i32, spheres: &[Sphere]) -> Color {
	let mut closest_t = f64::MAX;
	let mut closest_sphere: Option<&Sphere> = None;
	
	for sphere in spheres {
		let (t1, t2) = intersect_ray_sphere(o, d, &sphere);
		
		if is_closest_intersection(t1, start, end, closest_t) {
			closest_t = t1;
			closest_sphere = Some(sphere);
		}
		
		if is_closest_intersection(t2, start, end, closest_t) {
			closest_t = t2;
			closest_sphere = Some(sphere);
		}
	}
	
	return match closest_sphere {
		None => Color::White,
		Some(sphere) => sphere.color
	}
}

fn intersect_ray_sphere (origin: &Vec3<f64>, direction: &Vec3<f64>, sphere: &Sphere) -> (f64, f64) {
	let r = sphere.radius;
	let co = origin - &sphere.origin; // origin -> sphere 'c'entre
	
	let a = direction.dot(&direction);
	let b = 2. * co.dot(&direction);
	let c = co.dot(&co) - r*r;
	
	let discriminant = b*b - 4.*a*c;
	
	if discriminant < 0. {
		return (0.,0.);
	}
	
	return (
		((0. - b) + discriminant.sqrt()) / (2. * a),
		((0. - b) - discriminant.sqrt()) / (2. * a)
	);
}

fn render() {}
    // Image
	/*
	let aspect_ratio = 16.0 / 9.0;
	
	let image_width = 400.;
	let image_height = f64::max(1.0, image_width / aspect_ratio);
	
	let viewport_height = 2.0;
	let viewport_width = viewport_height * (image_width / image_height);
	
	eprintln!("aspect_ratio: {aspect_ratio}");
	eprintln!("image_width: {image_width}");
	eprintln!("image_height: {image_height}");
	eprintln!("viewport_height: {viewport_height}");
	eprintln!("viewport_width: {viewport_width}");
	
	// Render
	
	println!("P3");
	println!("{image_width} {image_height}");
	println!("255");
	
	let render_height = image_height as u32;
	let render_width = image_width as u32;
	
	for j in 0..render_height {
		for i in 0..render_width {
			
			let r = f64::from(i) / f64::from(render_width - 1);
			let g = 0.;
			let b = f64::from(j) / f64::from(render_height - 1);
			
			let color = Vec3 {x: r, y: g, z: b};
			
			write_color(color);
		}
	}
}*/
