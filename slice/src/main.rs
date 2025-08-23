fn main() {
    test1();

    test2();

    test3();

    test4();
}

fn test1() {
    let s = String::from("test1 hello");

    let index = first_word(&s);

    println!("첫 번째 공백 인덱스: {}", index);

    // 새로운 String에 직접 문자 복사.
    let mut first_word = String::new();

    for (i, c) in s.chars().enumerate() {
        if i == index {
            break;
        }
        first_word.push(c);
    }

    println!("첫 번째 단어: {}", first_word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn test2() {
    let s = String::from("test2 hello");

    let len = s.len();

    println!("문자열 총 길이 : {}", len);

    let slice1 = &s[0..5];
    let slice2 = &s[..5];

    println!("첫 인덱스 관련 : {} , {}", slice1, slice2);

    let slice1 = &s[6..len];
    let slice2 = &s[6..];

    println!("마지막 인덱스 관련 : {} , {}", slice1, slice2);

    let slice1 = &s[0..len];
    let slice2 = &s[..];

    println!("앞 뒤 인덱스 관련 : {} , {}", slice1, slice2);
}

fn test3() {
    let s = String::from("test3 hello");

    let first_s = first_word_fix(&s);

    println!("첫 번째 단어 : {}", first_s);
}

fn first_word_fix(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn test4() {
    let my_string = String::from("hello world");

    // first_word 함수에 String 타입으로부터 생성한 문자열 슬라이스를 전달한다.
    let word = first_word_in_slice(&my_string[..]);

    println!("String을 문자열 슬라이스로 전달 : {}", word);

    let my_string_literal = "hello world";

    // first_word함수에 문자열 리터럴의 슬라이스를 전달한다.
    let word = first_word_in_slice(&my_string_literal[..]);

    println!("문자열 리터럴을 슬라이스로 전달 : {}", word);

    // 문자열 리터럴은 이미 문자열 슬라이스이기 때문에
    // 아래의 코드는 슬라이스 문법 없이도 정상적으로 동작한다.
    let word = first_word_in_slice(my_string_literal);

    println!("문자열 리터럴로 전달 : {}", word);
}



fn first_word_in_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
