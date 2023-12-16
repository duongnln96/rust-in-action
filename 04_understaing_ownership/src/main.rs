#[warn(dead_code)]
fn calculate_length(s: &String) -> usize {
    s.len()
}

#[warn(dead_code)]
fn change(some_string: &mut String) {
    some_string.push_str(", World!")
}

// slice type
#[warn(dead_code)]
fn first_word_1(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

#[warn(dead_code)]
fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[warn(dead_code)]
fn first_word_3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s1 = String::from("Hello  WORLD Z");

    let word = first_word_2(&s1);

    println!("the first word is: {}", word);

    s1.clear();

    // {
    //     let r1 = &mut s1;
    //     r1.push_str(", world");
    // }
    // let r2 = &mut s1;
    // println!("new string is {}", r2);

    // let r1 = &s1;
    // let r2 = &s1;
    // println!("{} {}", r1, r2);

    // let r3 = &mut s1;
    // println!(" {}", r3);
}
