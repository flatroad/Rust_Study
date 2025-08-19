fn main() {
    case_loop();
    case_while();
    case_for();
}

fn case_loop() {
    println!("loop case");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result : {}", result);
}

fn case_while() {
    println!("\nwhile case");

    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    println!("발싸");
}

fn case_for() {
    println!("\nfor case");

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    for element in arr.iter() {
        println!("요소의 값: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }

    println!("발싸");
}
