fn main() {
    // 깊은 복사
    test1();
    // 매개변수 소유권 이전
    test2();
    // 함수 리턴 소유권 이전
    test3();
    // 매개변수 소유권 리턴으로 다시 원래 변수 재할당.
    test4();
    // 참조 대여
    test5();
    // 가변 대여 참조
    test6();
    // 에러 케이스 ( 컴파일러가 죽은 참조를 컴파일 시 찾아내 오류 처리)
    test7();
}

fn test1() {
    let s1 = String::from("test1");
    let s2 = s1.clone();

    println!("{}", s1);
    
    println!("{}", s2);
}

fn test2() {
    let s = String::from("test2");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn test3() {
    let s1 = gives_ownership();

    let s2 = String::from("test3-2");

    let s3 = takes_and_gives_back(s2);

    println!("{}", s1);
    println!("{}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("test3-1");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn test4() {
    let s1 = String::from("test4");

    let (s2, len) = calculate_length(s1);

    println!("'{}'의 길이는 {}입니다.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn test5() {
    let s1 = String::from("test5");

    let len = reference_calculate_length(&s1);

    println!("'{}'의 길이는 {}입니다.", s1, len);
}

fn reference_calculate_length(s: &String) -> usize {
    s.len()
}

fn test6() {
    let mut s1 = String::from("test6");

    let len = reference_mut_calculate_length(&mut s1);

    println!("'{}'의 길이는 {}입니다.", s1, len);
}

fn reference_mut_calculate_length(s: &mut String) -> usize {
    s.push_str(", hello");
    s.len()
}

fn test7() {
//    let reference_to_noting = dangle();
    println!(" test7은 오류 케이스 주석처리");
}

//fn dangle() -> &String {
//    let s = String::from("hello");
//
//    &s
//}
