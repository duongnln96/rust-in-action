# Smart Pointers

A *pointer* is a general concept for a variable that contains an address in memory.

The most common kind of pointer in Rust is a reference.

*Smart pointers* are data structures that act like a pointer but also have additional metadata and capabilities.

This allow data to have multiple owners by keeping track of the number of owners and, when no owners remain, cleaning up the data.

Rust, with its concept of ownership and borrowing, has an additional difference between references and smart pointers: while references only borrow data, in many cases, smart pointers own the data they point to.

**Two important trails in *smart pointer*:**

- `Deref` trail allows an instance of the smart pointer struct to behave like a reference so you can write your code to work with either references or smart pointers.

- `Drop` trail allows to customize the code that’s run when an instance of the smart pointer goes out of scope.

**The most common *smart pointers* in the standard library:**

- `Box<T>`, for allocating values on the heap

- `Rc<T>`, a reference counting type that enables multiple ownership

- `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time.

## Using `Box<T>` to Point to Data on the Heap

It'll use most often in these situations:

- When having a type whose size can not be know at complie time and wanting to use a value of that type in a context that requires an exact size.

- When having a large amount of data and wanting to transfer ownership but ensure the data will not be copied.

- When wanting to own a value and just care only that it's a type that implements a particular trail rather than being of a specific type.

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

- References are indicated by the `&` symbol and borrow the value they point to.

- Dereferences are indicated by the `*` symbol. `*y <-> *(y.deref())`
