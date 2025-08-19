fn main() {
	println!("안녕하세요");

	let a = 5;

	let b = {
		let x = 3;
		x + 1
	};

	println!("a의 값: {}", a);
	println!("b의 값: {}", b);
	
	another_function(5, 6);

	let c = five();

	println!("c의 값: {}", c);

	let d = plus_one(5);

	println!("d의 값: {}", d);
}

fn another_function(x: i32, y: i32) {
	println!("x의 값: {}", x);
	println!("y의 값: {}", y);
}

fn five() -> i32 {
	5
}

fn plus_one(x: i32) -> i32 {
	x + 1
}
