use std::collections::HashMap;

fn main() {
    let a = 10;
    let b = 15;
    println!("Hello, world! {} {}", a, b);
    // ! -> macro 

    // VARIABLES & DATA TYPES (u8, i8, f8, bool)
    // unsigned int (u8, u16, u32, u64, u128) 
    let unsigned: u8= 10;

    // signed int
    let signed: i8 = -10;

    // float
    let float: f32 = 1.2;

    // char
    let letter = "c123";
    let emoji = "\u{1F600}"; // :D

    // boolean
    let is_true: bool = true;

    // &&, ||, !

    // ARRAYS
    let arr1: [u8; 3] = [1, 2, 3];
    let arr2: [u8; 5] = [100; 5]; // [100, 100, 100, 100, 100]
    println!("Length of arr1: {}", arr1.len());
    println!("{:?}", arr1); // print full array 
    println!("Length of arr2: {}", arr2.len());
    println!("{:?}", arr2);
    println!("Element 1 in arr1: {}", arr1[0]);

    // TUPLE (can have multiple data types unlinke array)
    let tup1: (u8, bool, f32) = (5, true, 5.3);
    let _tup2 = (3,5);
    // let mut tup3 = (..); // mut -> mutable
    println!("{:?}", tup1);
    println!("Element 1 in tup1: {}", tup1.0); // .<index>
    // Destructuring
    let (a, b, c) = tup1;

    // FUNCTIONS
    greet("Aniket");

    // MUTABILITY 
    // Everything is immutable, use "mut" to make it mutable
    // let mut x = "hi";
    // x = "bye";

    // SLICING ARRAY
    // NOTE: For array, we know length in compile time but for a slice we don't
    let arr = [0, 1, 2, 3];
    let slice = &arr[1 .. 3]; // 1, 2 (index 1 to 3(excluding))
    borrowing_slice(arr, slice);

    // STRINGS
    let str: &str = "Hello World";
    let mut string: String = String::from("Hello World!");
    let str_slice = &string[..6];
    println!("{}", str_slice.len());
    string.push('1'); // append 1 character
    string.push_str("Hi"); // append string
    string = string.replace("Hello", "Hi");
    println!("{}", string);

    // IF
    let n = 3;
    if n>0 {
        println!("greater than 0");
    } else if n==0 {
        println!("equal to 0");
    } else {
        println!("less than 0");
    }

    // FOR
    for i in 0..6 {
        println!("{}", i); // 0 to 5
    }

    // WHILE
    let mut i = 0;
    while i<4 {
        println!("{}", i);
        i+=1;
    }

    // NOTE:- No need of semicolon for return, break, continue

    // MATCH (switch)
    let i = 5;
    match i {
        0 => println!("0"),
        1 | 2 => println!("1 or 2"),
        3..=4 => println!("3 to 4"),
        _ => println!("Default")
    }

    // STRUCTS
    let name = String::from("Bird");
    let bird = Bird {name: name, attack:5};
    bird.print_name();

    // TRAITS
    println!("{} {}", bird.can_fly(), bird.is_animal());

    // ENUMS
    let a: MyEnum = MyEnum::A;
    let b: MyEnum = MyEnum::B(5);
    let c: MyEnum = MyEnum::C { x: 10, y: 20 };
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
    if let MyEnum::C{x, y} = c { // B(val) = b
        println!("{} {}", x, y);
    }

    // VECTOR
    let mut v: Vec<i64> = vec![1, 2, 3, 4, 5];
    println!("{}", v.len());
    println!("{}", v[0]);
    v.push(8);
    v.remove(0);
    println!("{:?}", v);

    // HASH MAPS
    let mut map = HashMap::new();
    map.insert(0, "Hi");
    map.insert(3, "Bye");
    println!("{:?}", map);

    // .get returns Option (Enum of 2 values, Some and None)
    match map.get(&0) { // Hi
        Some(str) => println!("{}", str),
        None => println!("Doesn't exist in map")
    }

    map.remove(&0);
}

// by default, all functions are private, pub -> public
pub fn greet(name: &str) -> () { // <return type>, () => unit type (void) 
    println!("Hi {}", name);
    // NOTE: RETURN STATEMENT IS SIMPLY A NORMAL STATEMENT WITHOUT ;
    // Eg: 
    // let x = 100;
    // x // return value
}

// Using array slice in a function
fn borrowing_slice(arr: [u8; 4], slice: &[u8]) { // no need to give size in case of slice
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("Length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}

// Structs
struct Bird { 
    name: String,
    attack: u64 
}
impl Bird { // to implement functions
    fn print_name(&self) {
        println!("{}", self.name);
    }
}

// Traits
// Rust doesn't support inheritance
// It only supports interfaces, which is basically trait
trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool {
        true // default return
    }
}
impl Animal for Bird { // extending trait
    fn can_fly(&self) -> bool {
        true
    }
}

// ENUMS
#[derive(Debug)]
enum MyEnum {
    A,
    B(i32),
    C{x: i32, y: i32}
}