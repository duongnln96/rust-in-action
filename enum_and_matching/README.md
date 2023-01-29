# Enum and Pattern Matching

## Enum

```Rust
enum IpAddrKind {
    V4,
    V6,
}

// access enum
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

// enum type
fn route(ip_kind: IpAddrKind) {}
```

Enum can has more than one type

```Rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

## Match

Rust provides pattern matching via the `match` keyword, which can be used like a C `switch`.

The power of `match` comes from the expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled.

```Rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

The `Option` enum is built-in enum. It has two variants: `Some` and `None`

It is defined by the standard library as follows:

```Rust
enum Option<T> {
    None,
    Some(T),
}
```

Example:

```Rust
fn main() {
    let x = Some(5);
    let y: Option<i32> = None;

    match x {
        Some(value) => println!("x has value: {}", value),
        None => println!("x has no value"),
    }

    match y {
        Some(value) => println!("y has value: {}", value),
        None => println!("y has no value"),
    }
}
```

The `Option` enum is commonly used in Rust when you want to return a value that may or may not exist, without using a null value. This helps to avoid null pointer exceptions and other errors that can occur with null values.

We can use `_` as the placeholder to catch-all partterns

```Rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // using _ to catch-all patterns
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
```

## Control Flow with `if let`

```Rust

// using match
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1, // we have to add _ => () after processing just one variant, which is annoying boilerplate code to add.
    }

// using if let
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
```
