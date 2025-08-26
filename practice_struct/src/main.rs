struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    // 구조체 생성.
    test1();
    // 함수로 구조체 생성.
    test2();
    // 구조체 갱신법 이용 생성.
    test3();
    // 튜플 구조체.
    test4();
    // 간단한 사각형 넓이 구하기.
    test5();
}

fn test1() {
    let test1 = User {
        email: String::from("test1@rust.com"),
        username: String::from("test1"),
        active: true,
        sign_in_count: 1,
    };

    println!("username: {}", test1.username);
    println!("email: {}", test1.email); 
    println!("active: {}", test1.active);
    println!("sign_in_count: {}\n", test1.sign_in_count);
}

fn test2() {
    let test2_1 = build_user1(String::from("test2_1"));
    let test2_2 = build_user2(String::from("test2_2"));

    println!("test2_1: {}", test2_1.username);
    println!("test2_2: {}", test2_2.username);   
}

fn build_user1(username: String) -> User {
    User {
        email: String::from("test1@rust.com"),
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user2(username: String) -> User {
    User {
        email: String::from("test1@rust.com"),
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn test3() {
    let test3 = User {
        email: String::from("test1@rust.com"),
        username: String::from("test1"),
        active: true,
        sign_in_count: 1,
    };

    let test3_copy = User {
        username: String::from("test3_copy"),
        ..test3
    };

    println!("username: {}", test3.username);
    println!("active: {}", test3.active);

    println!("username: {}", test3_copy.username);
    println!("active: {}", test3_copy.active);
}

fn test4() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black : {}, {}, {}", black.0, black.1, black.2);
    println!("origin : {}, {}, {}", origin.0, origin.1, origin.2);
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle {width, height}
    }
}

fn test5() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "사각형의 면적: {} 제곱 픽셀",
        area(&rect1)
    );

    println!("메소드를 이용한 사각형의 면적: {} 제곱 픽셀", rect1.area());

    let test5 = Rectangle::create(40, 50);
    println!("연관함수로 생성된 사각형의 면적 : {} 제곱 픽셀", test5.area());
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
