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