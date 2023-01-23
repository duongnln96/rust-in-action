fn main() {
    let x = 5; // imutable
    // let mut x = 5; // mutable
    // println!("The value of x is: {x}");

    // x = 6;
    // println!("The value of x is: {x}");

    // shadow

    // let x = x + 1;

    // {
    //     let x = x*2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x is: {x}");

    let result: i32 = plus_one(x);
    println!("The value of x is: {result}");
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}