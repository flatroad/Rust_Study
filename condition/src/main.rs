use std::io;

fn main() {
	println!("숫자를 입력해 주세요");
	
	let mut number = String::new();

	io::stdin().read_line(&mut number).expect("잘못된 입력입니다.");

	let number: i32 = number.trim().parse().expect("숫자가 아닙니다.");

	println!("number : {}", number);

	let number: i32 = if number > 0 {
		1
	} else if number < 0 {
		-1
	} else {
		0
	};
	
	println!("바뀐 number : {}", number);
}
