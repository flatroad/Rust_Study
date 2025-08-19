fn main() {
	let tup: (i32, f64, u8) = (500, 6.4, 1);

	println!("first : {} , second : {} , third : {}", tup.0, tup.1, tup.2);

	let (x, y, z) = tup;
	
	println!("x : {} , y : {} , z : {}", x, y, z);

	let a: [i32; 5] = [3; 5];

	let first = a[0] + 2;
	
	let second = a[1] + 3;

	println!("i0 : {} , i1 : {} , i2 : {}, i3 : {}", a[0], a[1], a[2], a[3]);

	println!("first : {} , second : {}", first, second);

	//println!("error : {}", a[10]);	
}
