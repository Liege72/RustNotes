// fn main() {
//     let x: i32 = 5;
//
//     assert_eq!(x, 5);
//     println!("Success!");
// }

// fn main() {
//     // this var is not initialized; will only give a warning;
//     // remove warning by adding '_' at the start of var name
//     let _x: i32;
// }

// fn main() {
//     // "mut" allows var to be mutated
//     let mut x: i32 = 1;
//     x = x + 2; // = 3
//
//     assert_eq!(x, 3);
//     println!("Success!");
// }

// fn main() {
//     // ^^ outer scope
//     let x: i32 = 10;
//     let y: i32 = 5;
//
//     // inner scope
//     {
//         // let y: i32 = 5;
//         println!("The value of x is {} and value of y is {}", x, y);
//     }
//
//     println!("The value of x is {} and value of y is {}", x, y);
// }

// fn main() {
//     define_x();
// }
// fn define_x() {
//     let x: &str = "hello";
//
//     println!("{}, world", x);
// }

// fn main() {
//     // you can define variables more than once depending on the scope you are in;
//     // the x in the outer scope is 5 and the x in the inner scope is 12; see below
//
//     let x: i32 = 5;
//
//     {
//         let x = 12;
//         assert_eq!(x, 12);
//     }
//
//     assert_eq!(x, 5);
//
//     let x = 42;
//     println!("{}", x) // prints "42"
// }

// fn main() {
//     let mut x: i32 = 1;
//     x = 7; // x = 7
//
//     // shadowing and re-binding
//     let x = x; // 7
//     // x += 3; // <<-- will not work because the new x is not mutable
//
//     let mut x = x;
//     x += 3; // <<-- works because it is mutable
//
//     // you can change the types of variables;
//     // note that this is just dropping the initial declaration of y = 4
//     let y = 4;
//     // shadowing
//     let y: &str = "I can also be bound to text!";
//
//     println!("Success!");
// }

// #[allow(unused_variables)]
// fn main() {
//     // you can also add the above line to ignore warnings;
//     // this is an additional option to adding '_' at the start
//     // of a variable name
//     let x = 1;
// }

// fn main() {
//     // tuple; you can define mut keyword in tuple
//     let (mut x, y) = (1, 2);
//     x += 2;
//
//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
//     println!("Success!");
// }

// fn main() {
//     let (x, y);
//     // ^^ is the same as code below
//     // let x;
//     // let y;
//
//     // you are basically ignoring the variable where ".." is placed
//     (x,..) = (3, 4); // 4 is ignored
//     [.., y] = [1, 2]; // 1 is ignored
//
//     assert_eq!([x, y], [3, 2]);
//     println!("Success!");
// }

// fn main() {
//     println!("hello world!");
// }

// SECTION: INTEGERS

// fn main() {
//     let x: i32 = 5;
//     // let mut y: u32 = 5; <<-- does not work because you cannot change the type of a var
//     let mut y = 5;
//
//     y = x;
//
//     let z = 10; // type of z?
//     // the type of z is i32
//
//     println!("Success!");
// }

// fn main() {
//     let v: u16 = 38_u8 as u16;
//     // v as u16 is equal to the value 38 (which is an u8) as an u16
//
//     println!("Success!");
// }

// fn main() {
//     let x: u32 = 5;
//     assert_eq!("u32".to_string(), type_of(&x));
//
//     println!("Success!");
// }
// // get the types of a given variable, return a string rep of the type
// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }

// fn main() {
//     assert_eq!(i8::MAX, 127);
//     assert_eq!(u8::MAX, 255);
//
//     println!("Success!");
// }

// fn main() {
//     // the code below will exceed the max value of an u8
//     // let v1: u8 = 251_u8 + 8_u8;
//     // let v2 = i8::checked_add(251, 8).unwrap();
//
//     let v1: u16 = 251_u16 + 8;
//     let v2: i16 = i16::checked_add(251, 8).unwrap();
//
//     println!("{}{}", v1, v2)
// }

// fn main() {
//     let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255 = 1579
//     assert!(v == 1597);
//
//     println!("Success!");
// }

// fn main() {
//     let x: f64 = 1_000.000_1; // f64
//     let y: f32 = 0.12; // f32
//     let z: f64 = 0.01_f64; // f64
//
//     assert_eq!(typeof(&x), "f64".to_string());
//     println!("Success!");
// }
// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }

// fn main() {
//     // assert!(0.1+0.2==0.3);
//     // the above will not work because 0.1 + 0.2
//     // is equal to ~0.30000000000001 and not .3 exactly
//
//     assert!(0.1_f32+0.2_f32==0.3_f32);
//     assert!(0.1 as f32+0.2 as f32==0.3 as f32);
//     // both of the above statements will work because f32 is less precise
//
//     println!("Success!");
// }

// fn main() {
//     let mut sum: i32 = 0; // will become -5
//
//     // iterations
//     // -3, -2, -1, 0, 1
//     // not 2 because 2 is excluded
//     for i in -3..2 {
//         sum += i;
//     }
//
//     assert!(sum == -5);
//
//     // will iterate through a and z; uses ascii
//     for c in 'a'..='z' {
//         println!("{}", c as u8);
//         // c = the actual letter
//         // c as u8 = the ascii value of the char
//     }
// }

// use std::ops::{Range, RangeInclusive};
// fn main() {
//     assert_eq!((1..5), Range{ start: 1, end: 5});
//     assert_eq!((1..=5), RangeInclusive::new(1,5));;
//
//     println!("Success!");
// }

// fn main() {
//     // Integer addition
//     assert!(1u32 + 2 == 3);
//
//     // Integer subtraction
//     assert!(1i32 - 2 == -1);
//     // assert!(1u8 - 2 == -1); <<-- will not work because u8 is unsigned and cannot be negative
//
//     assert!(3 * 50 == 150);
//
//     assert!(9.6 as f32 / 3.2 as f32 == 3.0 as f32);
//
//     assert!(24 % 5 == 4);
//
//     // short-curcuiting boolean logic
//     assert!(true && false == false);
//     assert!(true || false == true);
//     assert!(!true == false);
//
//     // bitwise operations
//     println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
//     println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
//     println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
//     println!("1 << 5 is {}", 1u32 << 5);
//     println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
// }

// SECTION: CHAR, BOOL, AND UNIT

// use std::mem::size_of_val; // size of given value
// fn main() {
//     let c1 = 'a';
//     assert_eq!(size_of_val(&c1), 4);
//
//     println!("{}", size_of_val(&c1)); // 'a' is 4 bytes, so result will be 4
//
//     let c2 = '*';
//     assert_eq!(size_of_val(&c2), 4);
//     // same here ^^^
//
//     println!("Success!");
// }

// fn main() {
//     let c1 = '*';
//     print_char(c1);
// }
// fn print_char(c: char) {
//     println!("{}", c);
// }

// fn main() {
//     let _f = false;
//
//     let t = true;
//     if t {
//         println!("Success!");
//     }
// }

// fn main() {
//     let f = false;
//     let t = true && false; // false
//     assert_eq!(t, f);
//
//     println!("Success!");
// }

//fn main() {
//     // unit type; 0 bytes; holds no value; usually returned when fn returns no value
//     let _v: () = ();
//
//     let v = (2, 3);
//     assert_eq!(_v, implicitly_ret_unit());
//
//     println!("Success!");
// }
// fn implicitly_ret_unit() {
//     println!("I will return a ()");
// }
// fn explicitly_ret_unit() -> () {
//     println!("I will return a ()");
// }

// use std::mem::size_of_val;
// fn main() {
//     let unit: () = ();
//     assert!(size_of_val(&unit) == 0);
//
//     println!("Success!");
// }

// SECTION: STATEMENTS AND EXPRESSIONS

// fn main() {
//     let x = 5u32;
//
//     let y = {
//         let x_squared = x * x;
//         let x_cube = x_squared * x;
//
//         // this expression will vbe assigned to 'y'
//         x_cube + x_squared + x
//     };
//
//     let z = {
//         // the semicolon supresses this expression and '()' is assigned to 'z'
//         2 * x;
//     };
//
//     // notice that the last line in the 'y' statement does not contain a semicolon,
//     // while the last line in the 'z' statement does; the semicolon suppresses the assignment
//     // and will make the var return '()' aka a unit that contains 0 bytes
//
//     println!("x is {:?}", x);
//     println!("y is {:?}", y);
//     println!("z is {:?}", z);
// }

// fn main() {
//     let v = {
//         let mut x = 1;
//         x += 2; // 3
//         x
//     };
//
//     assert_eq!(v, 3);
//
//     println!("Success!");
// }

// fn main() {
//     // let v = (let x = 3); <<-- invalid syntax
//
//     let v = {
//         let x = 3;
//         x
//     };
//     // ^^ valid syntax
//
//     assert!(v == 3);
//
//     println!("Success!")
// }

// fn main() {
//     let s = sum(1, 2);
//     assert_eq!(s, 3);
//
//     println!("Success!");
// }
// fn sum(x: i32, y: i32) -> i32 {
//     x + y
//     // ending this with a semicolon will return a unit type
//     // ending this without a semicolon will return an i32
// }

// SECTION: FUNCTIONS

// fn main() {
//     let (x, y) = (1, 2);
//     let s = sum(x, y);
//
//     assert_eq!(s, 3);
//
//     println!("Success!");
// }
// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }

// fn main() {
//     print();
// }
// fn print() -> () { // "-> ()" is assumed already and is not needed
//     println!("Success!");
// }

// fn main() {
//     never_return();
//
//     println!("Failed");
// }
// fn never_return() -> ! {
//     panic!(); // kills the program
// }

// fn main() {
//      println!("Success");
// }
// fn get_option(tp: u8) -> Option<i32> {
//     match tp {
//         1 => {
//             // TODO
//         }
//         _ => {
//             // TODO
//         }
//     };
//
//     never_return_fn();
// }
// fn never_return_fn() -> ! {
//     // panic!();
//     // or
//     // unimplemented!();
//     // or
//     todo!();
// }

// fn main() {
//     let b = false;
//
//     let _v = match b {
//         true => 1,
//         false => {
//             println!("Success!");
//             panic!("we have no value for `false`, but we can panic!");
//         }
//     };
//
//     println!("Exercise failed if printing out this line!");
// }

// SECTION: OWNERSHIP

// fn main() {
//     let x: String = String::from("hello, world");
//
//     // the commented code below would not work because 'x' is now owned by 'y' and therefore
//     // cannot be used in the println macro
//     // let y: String = x;
//     // println!("{},{}", x, y);
//
//     // however, if you clone 'x' it will work
//     let y: String = x.clone();
//     println!("{},{}", x, y);
// }

// fn main() {
//     let s1 = String::from("hello, world");
//     let s2 = take_ownership(s1);
//
//     println!("{}", s2);
// }
// fn take_ownership(s: String) -> String {
//     println!("{}", s);
//     s
// }

// fn main() {
//     let s = give_ownership();
//     println!("{}", s);
// }
//
// fn give_ownership() -> String {
//     let s: String = String::from("hello, world");
//     let _s = s.as_bytes();
//     // Convert String Vec
//     let _s = s.as_bytes();
//     s
// }

// fn main() {
//     let s = String::from("hello world");
//
//     print_str(s.clone()); // clone is used so s can be used later in the function
//
//     println!("{}", s);
// }
// fn print_str(s: String) {
//     println!("{}", s);
// }

// fn main() {
//     let x = (1, 2, (), "hello");
//     let y = x;
//     println!("{:?}, {:?}", x, y);
// }

// fn main() {
//     let s = String::from("hello, ");
//
//     let mut s1 = s;
//
//     s1.push_str("world");
//
//     println!("Success!");
// }

// fn main() {
//     let x = Box::new(5); // box makes it heap allocated
//
//     let mut y = Box::new(1);
//
//     *y = 4;
//     // * is the "dereference" operator; it gets the value from pointer
//
//     assert_eq!(*x, 5);
//     println!("Success!");
// }

// fn main() {
//     #[derive(Debug)]
//     struct Person {
//         name: String,
//         age: Box<u8>, // box makes it heap allocated
//     }
//
//     let person = Person {
//         name: String::from("Alice"),
//         age: Box::new(20),
//     };
//
//     // `name` is moved out the person, but `age` is referenced
//     let Person { name, ref age } = person;
//
//     // "ref" keyword basically keeps the variable "age" under the ownership of the "person" var;
//     // because "name" is not referenced in the deconstructor above, it is under new ownership
//
//     println!("The person's age is {}", age);
//
//     println!("The person's name is {}", name);
//
//     // Error! borrow of partially moved value: `person` partial move occurs
//     // println!("The person struct is {:?}", person);
//
//     // `person` cannot be used but `person.age` can be used as it is not moved
//     println!("The person's age from person struct is {}", person.age);
// }

// fn main() {
//     let t = (String::from("hello"), String::from("world"));
//
//     let _s: String = t.0;
//
//     // println!("{:?}", t.0); // <<-- does not work because t.0 is under the ownership of _s
//     println!("{:?}", t.1); // this does work because t.1 is still under the ownership of t
// }

// fn main() {
//     let t = (String::from("hello"), String::from("world"));
//
//     let (s1, s2) = t.clone();
//
//     println!("{:?}, {:?}, {:?}", s1, s2, t);
// }

// fn main() {
//     let mut s = String::from("hello");
//
//     change(&mut s);
// }
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main () {
//     // let reference_to_nothing = dangle(); // <<-- does not work
//     let reference_to_nothing = no_dangle();
// }
// the function below will not work because you are creating a variable and returning
// its path in memory, but because the function ends, the path leads nowhere and is therefore
// dangling
// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
// }
// the function below will work because it is returning the actual value of s, not its
// pointer in the heap
// fn no_dangle() -> String {
//     let s = String::from("hello");
//     s
// }

// fn main() {
//     let x = 5;
//     let p = &x;
//
//     println!("the memory address of x is {:p}", p)
// }

// fn main() {
//     let x = 5;
//     let y = &x;
//
//     assert_eq!(5, *y);
//     // "*y" basically means: go to the location of y and get the value
//
//     println!("Success!");
// }

// fn main() {
//     let mut s = String::from("hello, ");
//
//     borrow_object(&s);
//
//     println!("Success!");
// }
// fn borrow_object(s: &String) {}

// fn main() {
//     let mut s = String::from("hello, ");
//
//     push_str(&mut s);
//
//     println!("Success!");
// }
//
// fn push_str(s: &mut String) {
//     s.push_str("world");
// }

// fn main() {
//     let mut s = String::from("hello, ");
//
//     let p = &mut s;
//
//     p.push_str("world");
//
//     println!("Success!");
// }

// fn main() {
//     let c = '*';
//
//     let r1 = &c;
//     let ref r2 = c;
//
//     assert_eq!(*r1, *r2);
//
//     assert_eq!(get_addr(r1), get_addr(r2));
//
//     println!("Success!");
// }
// fn get_addr(r: &char) -> String {
//     format!("{:p}", r) // format returns a string, while println returns () and prints
// }

// fn main() {
//     let mut s = String::from("hello");
//
//     // the code below will not work because you can only have one mutable reference
//     // of a variable at a time
//     // let r1 = &mut s;
//     // let r2 = &mut s;
//
//     // the code below will work because the mut keyword is left out; you can have as many
//     // immutable references as you would like, just one mutable though
//     let r1 = &s;
//     let r2 = &s;
//
//     println!("{} {}", r1, r2);
//
//     println!("Success!");
// }

// fn main() {
//     let mut s = String::from("hello, "); // added mut keyword so code below works
//
//     borrow_object(&mut s);
//
//     println!("Success!");
// }
// fn borrow_object(s: &mut String) { }

// fn main() {
//     let mut s = String::from("hello, ");
//
//     borrow_object(&s);
//
//     s.push_str("world");
//
//     println!("Success!");
// }
// fn borrow_object(s: &String) { }

// fn main() {
//     let mut s = String::from("hello");
//
//     let r1 = &mut s;
//     r1.push_str("world");
//
//     let r2 = &mut s;
//     r2.push_str("!");
//
//     // println!("{}", r1);
//     // comment out the line above because r1 and r2 are both mutable references; you cannot
//     // have two mutable references at once; because r1 is not used after r2 is created, there
//     // will be no issues; if it was used after r2s creation, the program would not compile because
//     // there would be two mutable references
// }

// fn main() {
//     let mut s = String::from("hello, ");
//
//     let r1 = &mut s;
//     let r2 = &mut s;
//
//     // adding the line below would cause the program to error
//     // println!("{}", r1);
//     // or
//     // println!("{}, {}", r1, r2);
//
//     // you cannot use r1 and r2 at the same time!
// }

// fn main() {
//     let s: &str = "hello, world";
//
//     println!("Success!");
// }

// fn main() {
//     let s: Box<str> = "hello, world".into();
//     greetings(&s);
//
//     // or
//     let s = "hello, world";
//     greetings(s);
// }
// fn greetings(s: &str) {
//     println!("{}", s);
// }

// fn main() {
//     // this is an empty string type; note that "" would not work
//     let mut s = String::from("");
//
//     s.push_str("hello, world");
//     s.push('!');
//
//     assert_eq!(s, "hello, world");
//
//     println!("{}", s);
//
//     println!("Success!");
//
//     // the whole point of using the "String" type is that they can be mutated as shown above
// }

// fn main() {
//     let mut s = String::from("hello");
//     s.push(',');
//     s.push_str(" world");
//     s += "!";
//
//     println!("{}", s);
// }

// fn main() {
//     let mut s = String::from("I like dogs");
//
//     // allocate new memory and store the modified string there
//     let s1 = s.replace("dogs", "cats");
//
//     assert_eq!(s1, "I like cats");
//
//     println!("Success!");
// }

// fn main() {
//     let s1 = String::from("hello, ");
//     let s2 = String::from("world1");
//
//     let s3 = s1 + s2.as_str(); // String -> &str
//     // or
//     // let s3 = s1 + &s2;
//
//     assert_eq!(s3, "hello, world");
//     println!("{}", s3);
// }

// fn main() {
//     let s = "hello, world";
//     greetings(s.to_string()); // &str -> String
//     // or
//     // greetings(String::from(s));
// }
// fn greetings(s: String) {
//     println!("{}", s);
// }

// fn main() {
//     let s = "hello, world".to_string();
//
//     let s1: &str = &s; // &String -> &str
//     // or
//     // let s1 = s.as_str();
//
//     println!("Success!");
// }

// fn main() {
//     let byte_escape = "I'm writing Ru\x73x74!";
//     println!("What are you doing\3F (\\x3F means ?) {}", byte_escape);
//
//     // escape chars and stuff; not super important
// }

// fn main() {
//     let raw_str = r"Escapes don't work here: \x3F \u{211D}";
//     // won't be converted to ascii                 ^^^^^^^^^^^^^^
// }

// fn main() {
//     // keep in mind that &str is a string reference, not an actual string; this means that when
//     // you slice with [x..y], you are slicing bytes, not characters in a string
//
//     let s1 = String::from("hi,#$");
//     let h = &s1[0..1];
//     assert_eq!(h, "h");
//
//     let h1 = &s1[3..4];
//     assert_eq!(h1, "#");
//
//     println!("{}", h1);
//
//     println!("Success!");
// }

// fn main() {
//     for c in "abcd".chars() {
//         println!("{}", c)
//     }
// }

//  SECTION: ARRAYS

// the function below would not work because the exact size of the array is not known
// at compile time
// fn init_arr(n: i32) {
//     let arr = [1; n];
// }

// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//
//     assert!(arr.len() == 5);
//
//     println!("Success!");
// }

// fn main() {
//     let arr0 = [1, 2, 3];
//     let arr: [_; 3] = ['a', 'b', 'c'];
//
//     assert!(std::mem::size_of_val(&arr) == 12);
//     // 12 because chars are 4 bytes each, 3 chars are in arr; 3 * 4 = 12
//
//     println!("Success!");
// }

// fn main() {
//     let list: [i32; 100] = [1; 100];
//
//     assert!(list[0] == 1);
//     assert!(list.len() == 100);
//
//     println!("Success!");
// }

// fn main() {
//     // let _arr: [i32; 3] = [1, 2, '3']; // <<-- will not work because all elements of an array
//     // must be of the same type
//
//     // this one does work!
//     let _arr = [1, 2, 3];
//
//     println!("Success!");
// }

// fn main() {
//     let arr = ['a', 'b', 'c'];
//
//     let ele = arr[0];
//
//     assert!(ele == 'a');
//
//     println!("Success!");
// }

// fn main() {
//     let names = [String::from("Sunfei"), "Sunface".to_string()];
//
//     // "get" returns an Option<T>, its safe to use
//     let name0 = names.get(0).unwrap();
//
//     // but index is not safe
//     let _name1 = &names[1];
//
//     println!("Success!");
// }

// fn main() {
//     let arr = [1, 2, 3];
//     let s1 = &arr[0..2]; // = &[1, 2]
//
//     let s2 = "hello, world" as &str;
//
//     println!("Success!");
// }

// fn main() {
//     let arr = ['a', 'b', 'c'];
//
//     let slice = &arr[..2];
//     // "slice" size
//     // pointer:     + 8 because it is an usize
//     // size:        + 8 because it is an usize; note that the size of the elements within
//     //                  the array do not matter
//     // total size of "slice" in bytes is: 12
//
//     assert!(std::mem::size_of_val(&slice) == 16);
//
//     println!("Success!");
// }

// fn main() {
//     let arr = [1, 2, 3, 4, 5];
//     let slice = &arr[1..4];
//
//     assert_eq!(slice, &[2, 3, 4]);
//
//     println!("Success!");
// }

// fn main() {
//     let s = String::from("hello");
//
//     let slice1 = &s[0..2];
//
//     let slice2 = &s[..2];
//
//     assert_eq!(slice1, slice2);
//
//     println!("Success!");
// }

// fn main() {
//     // note that slicing vars of type &str will slice in terms of bytes, not chars within the &str
//
//     let s = "你 好 世 界";
//     let slice = &s[0..3]; // = "你"
//     assert!(slice == "你");
//
//
//     // code below will not work because you can only slice &str, not a String
//     // let s1 = String::from("a b c d");
//     // let slice1 = s[0..2];
//     // assert!(slice1 == "a");
//
//     println!("Success!");
// }

// fn main() {
//     let mut s = String::from("Hello world");
//
//     let word = first_word(&s);
//     println!("the first word is: {}", word);
//
//     s.clear(); // error
//
//     // the line below was moved up below the "word" variable because if it remained down here,
//     // the program would not run; this is because you would be using the immutable reference in the
//     // "word" variable, then using a mutable reference when applying the clear function to the "s"
//     // variable; when the println macro is moved above the mutable reference, everything works!
//     // println!("the first word is: {}", word);
// }
// fn first_word(s: &str) -> &str {
//     &s[..1]
// }

// fn main() {
//     let _t0 = (0, -1);
//
//     let _t1 = (0, (-1, 1));
//
//     let t = (1u8, 2u16, 3i64, "hello", String::from(", world"));
//
//     println!("Success!");
// }

// fn main() {
//     let t = ("i", "am", "sunface");
//
//     assert_eq!(t.2, "sunface");
//
//     println!("Success!");
// }

// fn main() {
//     let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
//     println!("too long tuple: {:?}", too_long_tuple);
//
//     // this errors because tuple contains more than 12 values and therefore cannot be printed
// }

// fn main() {
//     let tup = (1, 6.4, "hello");
//
//     let (x, z, y) = tup;
//
//     assert_eq!(x, 1);
//     assert_eq!(y, "hello");
//     assert_eq!(z, 6.4);
//
//     println!("Success!");
// }

// fn main() {
//     let (x, y, z);
//
//     (y, z, x) = (1, 2, 3);
//
//     assert_eq!(x, 3);
//     assert_eq!(y, 1);
//     assert_eq!(z, 2);
//
//     println!("Success!");
// }

// fn main() {
//     let (x, y) = sum_multiply((2, 3));
//
//     assert_eq!(x, 5);
//     assert_eq!(y, 6);
//
//     println!("Success!");
// }
// fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
//     (nums.0 + nums.1, nums.0 * nums.1)
// }

// SECTION: STRUCTS

// struct Person {
//     name: String,
//     age: u8,
//     hobby: String
// }
// fn main() {
//     let age = 30;
//     let p = Person {
//         name: String::from("sunface"),
//         age,
//         hobby: String::from("video games"),
//     };
//
//     println!("Success!");
// }

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
// fn main() {
//     let v = Point(0, 127, 255);
//
//     check_color(v);
//
//     println!("Success!");
// }
// fn check_color(p: Point) {
//     let Point(x, _, z) = p;
//     assert_eq!(x, 0);
//     assert_eq!(p.1, 127);
//     assert_eq!(z, 255);
// }

// struct Person {
//     name: String,
//     age: u8,
// }
// fn main() {
//     let age: u8 = 18;
//     let mut p = Person {
//         name: String::from("sunface"),
//         age,
//     };
//
//     p.age = 30;
//
//     p.name = String::from("funfei");
//
//     println!("Success!");
// }

// struct Person {
//     name: String,
//     age: u8,
// }
// fn main() {
//     println!("Success!");
// }
// fn build_person(name: String, age: u8) -> Person {
//     Person {
//         age,
//         name,
//     }
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// fn main() {
//     let u1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("sunface"),
//         active: true,
//         sign_in_count: 1
//     };
//
//     let u2 = set_email(u1);
//
//     println!("Succes!");
// }
// fn set_email(u: User) -> User {
//     User {
//         email: String::from("contact@im.dev"),
//         ..u // take the rest from the argument provided
//     }
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale), // print debug info to stderr and assign a value
//         height: 50,
//     };
//
//     dbg!(&rect1);
//
//     println!("{:?}", rect1); // print debug info to stdout
// }

// fn main() {
//     #[derive(Debug)]
//     struct Person {
//         name: String,
//         age: Box<u8>,
//     }
//
//     let person = Person {
//         name: String::from("Alice"),
//         age: Box::new(20),
//     };
//
//     // "name" is moved out of person, but "age" is referenced
//     let Person { name, ref age } = person;
//
//     println!("The persons age is {}", age);
// }

// #[derive(Debug)]
// struct File {
//     name: String,
//     data: String,
// }
// fn main() {
//     let f = File {
//         name: String::from("readme.md"),
//         data: "Rust By Practice".to_string()
//     };
//
//     let _name = f.name;
//     // let _name = f.name.clone();
//
//     // accessing "f" and using it below would not work because its ownership is under "_name" now
//     println!("{} {}", _name, f.data);
//
//     // if you were to clone "f" like in the commented "_name" var above, you would be able to
//     // use the "f" var like in the line below
//     // println!("{} {} {:?}", _name, f.data, f);
// }

// SECTION: ENUMS

// enum Number {
//     Zero,   // 0
//     One,    // 1
//     Two,    // 2
// }
// enum Number1 {
//     Zero = 0,
//     One,
//     Two,
// }
// enum Number2 {
//     Zero = 5,
//     One,    // 6
//     Two,    // 7
// }
// fn main() {
//     // an enum variant can be converted to an integer by using "as"
//     assert_eq!(Number::One as u8, Number1::One as u8);
//     assert_eq!(Number1::One as u8, Number2::One as u8);
//
//     println!("{}", Number::One as u8);
//
//     println!("Success!");
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// fn main() {
//     let msg1 = Message::Move{ x: 1, y: 2};
//     let msg2 = Message::Write(String::from("hello, world"));
//
//     println!("Success!");
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// fn main() {
//     let msg = Message::Move{ x: 1, y: 1 };
//
//     if let Message::Move{x: a, y: b} = msg {
//         assert_eq!(a, b);
//     } else {
//         panic!("NEVER LET THIS RUN!");
//     }
//
//     println!("Success!");
// }

// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// fn main() {
//     let msgs: [Message; 3] = [
//         Message::Quit,
//         Message::Move { x:1, y:3 },
//         Message::ChangeColor(255, 255, 0),
//     ];
//
//     for msg in msgs {
//         show_message(msg)
//     }
// }
// fn show_message(msg: Message) {
//     println!("{:?}", msg)
// }

// fn main() {
//     let five: Option<i32> = Some(5);
//     let six = plus_one(five); // Some(6)
//     let none = plus_one(None);
//
//     if let Some(n) = six {
//         println!("{:?}", n);
//
//         println!("Success!");
//     } else {
//         panic!("NEVER LET THIS RUN!");
//     }
// }
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// fn main() {
//     let n = 5;
//
//     if n < 0 {
//         println!("{} is negative", n)
//     } else if n > 0 {
//         println!("{} is positive", n)
//     }
//     else {
//         println!("{} is zero", n);
//     }
// }

// fn main() {
//     let n = 5;
//
//     let big_n =
//         if n < 10 && n > -10 {
//             println!(", and is a small number, increase ten-fold");
//
//             10 * n
//         } else {
//             println!(", and is a big number, halve the number");
//
//             n / 2.0 as i32
//         };
//
//     println!("{} -> {}", n, big_n);
// }

// fn main() {
//     for n in 1..100 {
//         if n == 100 {
//             panic!("NEVER LET THIS RUN!");
//         }
//     }
//
//     println!("Success!");
// }

// fn main() {
//     let names = [String::from("liming"), String::from("hanmeimei")];
//     // this for loop will take ownership of the "names" variable unless you reference it using &
//     for name in &names {
//         println!("{}", name);
//     }
//     println!("{:?}", names);
//
//     let numbers = [1, 2, 3];
//     for n in numbers {
//         println!("{}", n);
//     }
//     println!("{:?}", numbers);
// }

// fn main() {
//     let a = [4, 3, 2, 1];
//
//     // i = the current index
//     // v = the current value
//     for (i, v) in a.iter().enumerate() {
//         println!("The {}th element is {}", i+1, v);
//     }
// }

// fn main() {
//     let mut n = 1;
//
//     while n < 10 {
//         if n % 15 == 0 {
//             println!("fizzbuzz");
//         } else if n % 3 == 0 {
//             println!("fizz");
//         } else if n % 5 == 0 {
//             println!("buzz")
//         } else {
//             println!("{}", n)
//         }
//
//         n += 1;
//     }
//
//     println!("n reached {}, so loop is over", n);
// }

// fn main() {
//     let mut n = 0;
//     for i in 0..=100 {
//         if n == 66 {
//             break;
//         }
//         n += 1;
//     }
//
//     assert_eq!(n, 66);
//
//     println!("Success!");
// }

// fn main() {
//     let mut n = 0;
//     for i in 0..=100 {
//         if n != 66 {
//             n += 1;
//             continue;
//         }
//         break;
//     }
//
//     assert_eq!(n, 66);
//
//     println!("Success!");
// }

// fn main() {
//     let mut count = 0u32;
//
//     println!("Let's count until infinity!");
//
//     // infinite loop
//     loop {
//         count += 1;
//
//         if count == 3 {
//             println!("three");
//
//             // skip the rest of this iteration
//             continue;
//         }
//
//         println!("{}", count);
//
//         if count == 5 {
//             println!("OK, that's enough");
//
//             break;
//         }
//     }
// }

// fn main() {
//     let mut counter = 0;
//
//     let result = loop {
//         counter += 1;
//
//         if counter == 10 {
//             break counter;
//             // breaks and returns the value of counter
//         }
//     };
//
//     assert_eq!(result, 20);
//
//     println!("Success!");
// }

// fn main() {
//     let mut count = 0;
//     'outer: loop {
//         'inner1: loop {
//             if count >= 20 {
//                 // this would break only the inner1 loop
//                 break 'inner1; // "break" also works
//             }
//             count += 2;
//         }
//
//         count += 5;
//
//         'inner2: loop {
//             if count >= 30 {
//                 // this breaks the outer loop
//                 break 'outer;
//             }
//
//             // this will continue the outer loop
//             continue 'outer;
//         }
//     }
//
//     assert!(count == 30);
//
//     println!("Success!");
// }

// SECTION: MATCH

// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }
// fn main() {
//     let dire = Direction::South;
//     match dire {
//         Direction::East => println!("East"),
//         Direction::South | Direction::North => {
//             println!("South or North")
//         },
//         _ => println!("West"),
//     }
// }

// fn main() {
//     let boolean = true;
//
//     let binary = match boolean {
//         true => 1,
//         false => 0,
//     };
//
//     assert_eq!(binary, 1);
//
//     println!("Success!");
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// fn main() {
//     let msgs = [
//         Message::Quit,
//         Message::Move{x:1, y:3},
//         Message::ChangeColor(255, 255, 0),
//     ];
//
//     for msg in msgs {
//         show_message(msg)
//     }
//
//     println!("Success!");
// }
// fn show_message(msg: Message) {
//     match msg {
//         Message::Move{x:a, y:b} => {
//             assert_eq!(a, 1);
//             assert_eq!(b, 3);
//         }
//         Message::ChangeColor(r, g, b) => {
//             assert_eq!(g,255);
//             assert_eq!(b, 0);
//         }
//
//         _ => println!("no data in these variants")
//     }
// }

// fn main() {
//     let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];
//
//     for ab in alphabets {
//         assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9'));
//         // if any fail, program will panic!
//     }
//
//     println!("Success!");
// }

// enum MyEnum {
//     Foo,
//     Bar
// }
// fn main() {
//     let mut count = 0;
//
//     let v = vec!(MyEnum::Foo,MyEnum::Bar,MyEnum::Foo);
//     for e in v {
//         // the below if statement will not work
//         // if e == MyEnum::Foo {
//         //     count += 1;
//         // }
//
//         // this one will
//         if matches!(e, MyEnum::Foo) {
//             count += 1;
//         }
//     }
//
//     assert_eq!(count, 2);
//
//     println!("Success!");
// }

// fn main() {
//     let o = Some(7);
//
//     // both the match and the if let statements work; if let is best practice in this case
//
//     match o {
//         Some(i) => {
//             println!("This is a really long string and `{:?`}", i);
//
//             println!("Success!");
//         }
//         _ => {}
//     }
//
//     // 7 is destructured into i
//     if let Some(i) = o {
//         println!("This is a really long string and `{:?`}", i);
//
//         println!("Success!");
//     }
// }

// enum Foo {
//     Bar(u8)
// }
// fn main() {
//     let a = Foo::Bar(1);
//
//     if let Foo::Bar(i) = a {
//         println!("foobar holds the value: {}", i);
//
//         println!("Success!");
//     }
// }

// enum Foo {
//     Bar,
//     Baz,
//     Qux(u32)
// }
// fn main() {
//     let a = Foo::Qux(10);
//
//     match a {
//         Foo::Bar =>  println!("match foo:bar"),
//         Foo::Baz =>  println!("match foo:baz"),
//         _ => println!("match others")
//     }
// }

// fn main() {
//     let age = Some(30);
//
//     // both the if let and match statements are shadowing the variable "age"; its basically
//     // becoming a new variable with the same name
//
//     if let Some(age) = age {
//         assert_eq!(age, 30);
//     }
//
//     match age {
//         Some(age) => println!("age is a new variable, its value is {}", age),
//         _ => { }
//     }
// }

// SECTION: PATTERNS

// fn main() { }
// fn match_number(n: i32) {
//     match n {
//         1 => println!("one"),
//         2 | 3 | 4 | 5 => println!("match 2 -> 5"),
//         6..=10 => println!("match 6 -> 10"),
//         _ => {
//             println!("match -infinite -> 0 or 11 -> +infinite")
//         }
//     }
// }

// struct Point {
//     x: i32,
//     y: i32
// }
// fn main() {
//     let p = Point { x: 0, y: 10 };
//
//     match p {
//         Point { x, y:0 } => println!("on the x axis at {}", x),
//         Point { x: 0..=5, y: y @ (10 | 20 | 30) } => println!("on the y axis at {}", y),
//         Point { x, y } => println!("on neither axis ({}, {})", x, y)
//     }
// }

// enum Message {
//     Hello { id: i32 }
// }
// fn main() {
//     let msg = Message::Hello { id: 5 };
//
//     match msg {
//         Message::Hello {
//             id: id @ 3..=7
//         } => println!("found an id in range [3, 7]: {}", id),
//         Message::Hello { id: newid@ (10 | 11 | 12) } => {
//             println!("found an id in another range [10, 12]: {}", newid)
//         }
//         Message::Hello { id } => println!("found other id {}", id)
//     }
// }

// fn main() {
//     let num = Some(4);
//     let split = 5;
//     match num {
//         Some(x) if x < split => assert!(x < split),
//         Some(x) => assert!(x >= split),
//         None => (),
//     }
//
//     println!("Success!");
// }

// fn main() {
//     let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);
//
//     match numbers {
//         (first, .., last) => {
//             assert_eq!(first, 2);
//             assert_eq!(last, 2048);
//         }
//     }
//
//     println!("Success!");
// }

// fn main() {
//     let mut v = String::from("hello,");
//     let r = &mut v;
//
//     match r {
//         value => value.push_str(" world!")
//     }
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50; };
//
//     assert_eq!(rect1.area(), 1500);
//
//     println!("Success!");
// }

// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }
// impl TrafficLight {
//     // either of the self declarations below will work
//     pub fn show_estate(self: &Self) {
//         println!("the current state is {}", self.color);
//     }
//     pub fn change_state(&mut self) {
//         self.color = "green".to_string();
//     }
// }
// fn main() {
//     println!("Success!");
// }

// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }
// impl TrafficLight {
//     pub fn new() -> TrafficLight {
//         TrafficLight {
//             color: String::from("red")
//         }
//     }
//     pub fn get_state(&self) -> &str {
//         &self.color
//     }
// }
// fn main() {
//     let light = TrafficLight::new();
//     assert_eq!(light.get_state(), "red");
//     println!("Success!");
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
// fn main() {
//     println!("Success!");
// }

use std::fmt::format;
use std::path::Component::ParentDir;
use std::str::from_utf8;

// #[derive(Debug)]
// enum TrafficLightColor {
//     Red,
//     Yellow,
//     Green,
// }
// impl TrafficLightColor {
//     fn color(&self) -> &str {
//         match self {
//             TrafficLightColor::Yellow => "yellow",
//             TrafficLightColor::Red => "red",
//             TrafficLightColor::Green => "green"
//         }
//     }
// }
// fn main() {
//     let c = TrafficLightColor::Yellow;
//
//     assert_eq!(c.color(), "yellow");
//
//     println!("Success!");
// }

// struct A;
// struct S(A);
// struct SGen<T>(T);
// fn reg_fn(_s: S) {}
// fn gen_spec_t(s: SGen<A>) {}
// fn gen_spec_i32(_s: SGen<i32>) {}
// fn generic<T>(_s: SGen<T>) {}
// fn main() {
//     reg_fn(S(A));
//     gen_spec_t(SGen(A));
//     gen_spec_i32(SGen(7));
//
//     generic::<char>(SGen('a'));
//     generic(SGen(7.7));
//
//     println!("Success!");
// }

// fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
//     a + b
// }
//
// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));
// }

// struct Point<T>{
//     x: T,
//     y: T,
// }
// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
//
//     println!("Success!");
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// fn main() {
//     let p = Point { x: 5, y: "hello".to_string() };
//
//     println!("Success!")
// }

// struct Val<T> {
//     val: T,
// }
// impl<T> Val<T> {
//     fn value(&self) -> &T {
//         &self.val
//     }
// }
// fn main() {
//     let x = Val { val: 3.0 };
//     let y = Val { val: "hello".to_string() };
//     println!("{}, {}", x.value(), y.value());
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y
//         }
//     }
// }
// fn main() {
//     let p1 = Point { x: 5, y: 10 };
//     let p2 = Point { x: "hello", y: "world" };
//
//     let p3 = p1.mixup(p2);
//
//     assert_eq!(p3.x, 5);
//     assert_eq!(p3.y, "world");
//
//     println!("Success!");
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }
// impl Point<f64> {
//     fn distance_from_origin(&self) -> f64 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }
// fn main() {
//     let p = Point { x: 5.0, y: 10.0 };
//     println!("{}", p.distance_from_origin());
// }

// SUBSECTION: CONST GENERICS

// struct Array<T, const N: usize> {
//     data: [T; N] // [f64; 2]
// }
// fn main() {
//     // all must be same type and same amount of elements
//     let arrays: [Array<i32, 3>; 3] = [
//         Array {
//             data: [1, 2, 3],
//         },
//         Array {
//             data: [1, 2, 3],
//         },
//         Array {
//             data: [4, 5, 6]
//         }
//     ];
//
//     let floats: [Array<f64, 2>; 3] = [
//         Array { data: [1.0, 2.0] },
//         Array { data: [3.0, 4.0] },
//         Array { data: [5.0, 6.0] },
//     ];
//
//     println!("Success!");
// }

// SECTION: TRAITS

// trait Hello {
//     fn say_hi(&self) -> String {
//         String::from("hi")
//     }
//
//     fn say_something(&self) -> String;
// }
// struct Student { }
// impl Hello for Student {
//     fn say_something(&self) -> String {
//         String::from("I'm a good student")
//     }
// }
// struct Teacher { }
// impl Hello for Teacher {
//     fn say_hi(&self) -> String {
//         String::from("Hi, I'm your new teacher")
//     }
//     fn say_something(&self) -> String {
//         String::from("I'm not a bad teacher")
//     }
// }
// fn main() {
//     let s = Student { };
//     assert_eq!(s.say_hi(), "hi");
//     assert_eq!(s.say_something(), "I'm a good student");
//
//     let t = Teacher { };
//     assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
//     assert_eq!(t.say_something(), "I'm not a bad teacher");
//
//     println!("Success!");
// }


// traits used below are described below:
// Debug: allows you to use "{:?}" in strings so you can get the vale of the variable
// PartialEq: allows you to use "==" operators to check if a variable is equal to another
// PartialOrd: allows you to use the ">" and "<" operators so you can see which variable
//             has bigger/smaller value
// #[derive(PartialEq, PartialOrd)]
// struct Centimeters(f64);
// #[derive(Debug)]
// struct Inches(i32);
// impl Inches {
//     fn to_centimeters(&self) -> Centimeters {
//         let &Inches(inches) = self;
//         Centimeters(inches as f64 * 2.54)
//     }
// }
// #[derive(Debug, PartialEq, PartialOrd)]
// struct Seconds(i32);
// fn main() {
//     let _one_second = Seconds(1);
//
//     println!("One second looks like: {:?}", _one_second);
//     let _this_is_true = (_one_second == _one_second);
//     let _this_is_true = (_one_second > _one_second);
//
//     let foot = Inches(12);
//
//     println!("One foot equals {:?}", foot);
//
//     let meter = Centimeters(100.0);
//
//     let cmp =
//         if foot.to_centimeters() < meter {
//             "smaller"
//         } else {
//             "bigger"
//         };
//
//     println!("One foot is {} than one meter.", cmp);
// }

// use std::ops;
// // allowing the type to be multiplied and setting the output to be the generic type of T
// fn multiply<T: std::ops::Mul<Output = T>>(a: T, b: T) -> T {
//     a * b // a.mul(b)
// }
// fn main() {
//     assert_eq!(6, multiply(2u8, 3u8));
//     assert_eq!(5.0, multiply(1.0, 5.0));
// }

// use std::ops;
// struct Foo;
// struct Bar;
// #[derive(PartialEq, Debug)]
// struct FooBar;
// #[derive(PartialEq, Debug)]
// struct BarFoo;
// // implementing the "Add" trait from the standard library
// // Foo + Bar -> Foo.add(Bar)
// impl ops::Add<Bar> for Foo {
//     type Output = FooBar;
//
//     fn add(self, _rhs: Bar) -> FooBar {
//         FooBar
//     }
// }
// impl ops::Sub<Bar> for Foo {
//     type Output = BarFoo;
//
//     fn sub(self, _rhs: Bar) -> BarFoo {
//         BarFoo
//     }
// }
// fn main() {
//     assert_eq!(Foo + Bar, FooBar);
//     assert_eq!(Foo - Bar, BarFoo);
//
//     println!("Success!");
// }

// trait Summary {
//     fn summarize(&self) -> String;
// }
// #[derive(Debug)]
// struct Post {
//     title: String,
//     author: String,
//     content: String,
// }
// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("The author of post {} is {}", self.title, self.author)
//     }
// }
// #[derive(Debug)]
// struct Weibo {
//     username: String,
//     content: String,
// }
// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{} published a weibo {}", self.username, self.content)
//     }
// }
// fn main() {
//     let post = Post {
//         title: "Popular Rust".to_string(),
//         author: "Sunface".to_string(),
//         content: "Rust is awesome!".to_string(),
//     };
//     let weibo = Weibo {
//         username: "sunface".to_string(),
//         content: "Weibo seems to be worse than Tweet".to_string(),
//     };
//
//     summary(&post);
//     summary(&weibo);
//
//     println!("{:?}", post);
//     println!("{:?}", weibo);
//
//     // prints the below text:
//     // The author of post Popular Rust is Sunface
//     // sunface published a weibo Weibo seems to be worse than Tweet
//     // Post { title: "Popular Rust", author: "Sunface", content: "Rust is awesome!" }
//     // Weibo { username: "sunface", content: "Weibo seems to be worse than Tweet" }
// }
// fn summary(a: &impl Summary) {
//     let output = a.summarize();
//     println!("{}", output);
// }

// struct Sheep { }
// struct Cow { }
// trait Animal {
//     fn noise(&self) -> String;
// }
// impl Animal for Sheep {
//     fn noise(&self) -> String {
//         "baah".to_string()
//     }
// }
// impl Animal for Cow {
//     fn noise(&self) -> String {
//         "moo".to_string()
//     }
// }
// fn random_animal(random_number: f64) -> Box<dyn Animal> {
//     if random_number < 0.5 {
//         Box::new(Sheep { })
//     } else {
//         Box::new(Cow { })
//     }
// }
// fn main() {
//     let random_number = 0.234;
//     let animal = random_animal(random_number);
//     println!("You've randomly chosen an animal, and it says {}", animal.noise());
// }

// fn main() {
//     assert_eq!(sum(1, 2), 3);
// }
// fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
//     x + y
// }

// struct Pair<T> {
//     x: T,
//     y: T,
// }
// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self {
//             x,
//             y,
//         }
//     }
// }
// impl<T: std::fmt::Debug + PartialOrd + PartialEq> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {:?}", self.x);
//         } else {
//             println!("The largest member is y = {:?}", self.y);
//         }
//     }
// }
// #[derive(Debug, PartialOrd, PartialEq)]
// struct Unit(i32);
// fn main() {
//     let pair = Pair::new(Unit(1), Unit(2));
//
//     pair.cmp_display();
// }

// trait Bird {
//     fn quack(&self) -> String;
// }
// struct Duck;
// impl Duck {
//     fn swim(&self) {
//         println!("Look, the duck is swimming");
//     }
// }
// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck.. oh sorry, the swan is flying")
//     }
// }
// impl Bird for Duck {
//     fn quack(&self) -> String {
//         "duck duck".to_string()
//     }
// }
// impl Bird for Swan {
//     fn quack(&self) -> String {
//         "swan swan".to_string()
//     }
// }
// fn main() {
//     let duck = Duck;
//     duck.swim();
//
//     let bird = hatch_a_bird(2);
//     assert_eq!(bird.quack(), "duck duck");
//
//     let bird = hatch_a_bird(1);
//     assert_eq!(bird.quack(), "swan swan");
//
//     println!("Success")
// }
// fn hatch_a_bird(species: u8) -> Box<dyn Bird> {
//     match species {
//         1 => Box::new(Swan),
//         2 => Box::new(Duck),
//         _ => panic!(),
//     }
// }

// trait Bird {
//     fn quack(&self);
// }
// struct Duck;
// impl Duck {
//     fn fly(&self) {
//         println!("Look, the duck is flying")
//     }
// }
// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck.. oh sorry, the swan is flying")
//     }
// }
// impl Bird for Duck {
//     fn quack(&self) {
//         println!("{}", "duck duck")
//     }
// }
// impl Bird for Swan {
//     fn quack(&self) {
//         print!("{}", "swan swan");
//     }
// }
// fn main() {
//     let birds: [&dyn Bird; 2] = [&Duck, &Swan];
//
//     for bird in birds {
//         bird.quack();
//
//         // will not work because only the methods from the Bird trait are in the v-table??
//         // the compiler cannot look it up at runtime
//         // bird.fly();
//     }
// }

// trait Draw {
//     fn draw(&self) -> String;
// }
// impl Draw for u8 {
//     fn draw(&self) -> String {
//         format!("u8: {}", self)
//     }
// }
// impl Draw for f64 {
//     fn draw(&self) -> String {
//         format!("f64: {}", self)
//     }
// }
// fn main() {
//     let x = 1.1f64;
//     let y = 8u8;
//
//     draw_with_box(Box::new(x));
//     draw_with_ref(&y);
//
//     println!("Success!");
// }
// fn draw_with_box(x: Box<dyn Draw>) {
//     x.draw();
// }
// fn draw_with_ref(x: &dyn Draw) {
//     x.draw();
// }

// trait Foo {
//     fn method(&self) -> String;
// }
// impl Foo for u8 {
//     fn method(&self) -> String { format!("u8: {}", *self) }
// }
// impl Foo for String {
//     fn method(&self) -> String { format!("string: {}", *self) }
// }
// fn static_dispatch<T: Foo>(a: T) {
//     a.method();
// }
// fn dynamic_dispatch(a: &dyn Foo) {
//     a.method();
// }
// fn main()
// {
//     let x = 5u8;
//     let y = "Hello".to_string();
//
//     static_dispatch(x);
//     dynamic_dispatch(&y);
// }

// trait MyTrait {
//     fn f(&self) -> Self;
// }
// impl MyTrait for u32 {
//     fn f(&self) -> Self { 42 }
// }
// impl MyTrait for String {
//     fn f(&self) -> Self { self.clone() }
// }
// fn my_function<T: MyTrait>(x: T) -> T {
//     x.f()
// }
// fn main() {
//     my_function(13_u32);
//     my_function(String::from("abc"));
//
//     println!("Success!");
// }

// trait MyTrait {
//     fn f(&self) -> Box<dyn MyTrait>;
// }
// impl MyTrait for u32 {
//     fn f(&self) -> Box<dyn MyTrait> { Box::new(42) }
// }
// impl MyTrait for String {
//     fn f(&self) -> Box<dyn MyTrait> { Box::new(self.clone()) }
// }
// fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait> {
//     x.f()
// }
// fn main() {
//     my_function(Box::new(13_u32));
//     my_function(Box::new(String::from("abc")));
//
//     println!("Success!");
// }

// ---

// fn main() {
//     let mut s = String::from("hello, ");
//     s.push_str("world");
//     s.push('!');
//
//     move_ownership(s.clone());
//
//     assert_eq!(s, "hello, world!");
//
//     println!("Success!");
// }
// fn move_ownership(s: String) {
//     println!("ownership of \"{}\" is moved here!", s)
// }

// fn main() {
//     let mut s = String::from("hello, world");
//
//     let slice1 = s.as_str();
//     assert_eq!(slice1, "hello, world");
//
//     let slice2 = &s[..5];
//     assert_eq!(slice2, "hello");
//
//     let slice3 = &mut s;
//     slice3.push('!');
//     assert_eq!(slice3, "hello, world");
//
//     println!("Success!");
// }

// fn main() {
//     // how many heap allocations are happening here?
//
//     let s = String::from("hello, world"); // 1
//
//     let slice = &s;
//
//     let s = slice.to_string(); // 2
//
//     assert_eq!(s, "hello, world");
//     println!("Success!");
// }

// fn main() {
//     let s = String::from("hello, *");
//     let slice1 = &s[..1];
//     assert_eq!(slice1, "h");
//
//     let slice2 = &s[7..8];
//     assert_eq!(slice2, "*");
//
//     for (i, c) in s.chars().enumerate() {
//         if i == 7 {
//             assert_eq!(c, '*');
//         }
//     }
// }

// use utf8_slice; // idk why its not working
// fn main() {
//     let s = "The 🚀 goes to the 🌑!";
//
//     let rocket = utf8_slice::slice(s, 4, 5);
//     // will equal "🚀"
// }

// fn main() {
//     let mut s = String::new();
//     s.push_str("hello");
//
//     // spells "hello" in ascii
//     let v = vec![104, 101, 108, 108, 111];
//
//     let s1 = String::from_utf8(v).unwrap();
//
//     assert_eq!(s, s1);
//     println!("Success!");
// }

// fn main() {
//     // you can predefine the amount of space you want a string to hold
//     let mut s = String::with_capacity(25);
//
//     println!("{}", s.capacity());
//
//     for _ in 0..2 {
//         s.push_str("hello"); // "hello", "hellohello"
//         println!("{}", s.capacity());
//     }
//
//     println!("Success!");
// }

// SECTION: VECTORS

// fn main() {
//     let arr: [u8; 3] = [1, 2, 3];
//
//     let v: Vec<u8> = Vec::from(arr);
//     is_vec(v);
//
//     let v = vec![1, 2, 3];
//     is_vec(v);
//
//     let v = vec!(1, 2, 3);
//     is_vec(v.clone());
//
//     // new empty vector with length of 0 and capacity of 0
//     let mut v1 = Vec::new();
//     is_vec(v1.clone());
//
//     for i in &v {
//         v1.push(*i);
//     }
//
//     assert_eq!(v, v1);
// }
// fn is_vec(v: Vec<u8>) { }

// fn main() {
//     let mut v1 = Vec::from([1, 2, 4]);
//     v1.pop();
//     v1.push(3);
//
//     let mut v2 = Vec::new();
//     v2.extend(&v1);
//
//     assert_eq!(v1, v2);
//
//     println!("Success!");
// }

// fn main() {
//     let arr = [1, 2, 3];
//     let v1 = Vec::from(arr);
//     let v2: Vec<i32> = arr.into();
//     assert_eq!(v1, v2);
//
//     let s = "hello".to_string();
//     let v1: Vec<u8> = s.into();
//
//     let s = "hello".to_string();
//     let v2 = s.into_bytes();
//     assert_eq!(v1, v2);
//
//     let s = "hello";
//     let v3 = Vec::from(s);
//     assert_eq!(v2, v3);
//
//     let v4: Vec<i32> = [0; 10].into_iter().collect();
//     assert_eq!(v4, vec!(0; 10));
//
//     println!("Success!");
// }

// fn main() {
//     let mut v = Vec::from([1, 2, 3]);
//     for i in 0..5 {
//         println!("{:?}", v.get(i));
//     }
//
//     for i in 0..5 {
//         match v.get(i) {
//             Some(e) => v[i] = e + 1,
//             None => v.push(i + 2)
//         }
//     }
//
//     assert_eq!(v, vec![2, 3, 4, 5, 6]);
//     println!("Success!");
// }

// fn main() {
//     let mut v = vec![1, 2, 3];
//
//     let slice1 = &v[..];
//     let slice2 = &v[0..v.len()];
//
//     assert_eq!(slice1, slice2);
//
//     let vec_ref = &mut v;
//     (*vec_ref).push(4);
//     let slice3 = &v[0..4];
//
//     assert_eq!(slice3, &[1, 2, 3, 4]);
//
//     println!("Success!");
// }

// fn main() {
//     // when a vectors capacity is specified, its contents are stored on stack,
//     // if capacity is increased, the vector will be reallocated to the heap, making things
//     // a little slower
//
//     let mut vec = Vec::with_capacity(10);
//
//     assert_eq!(vec.len(), 0);
//     assert_eq!(vec.capacity(), 10);
//
//     for i in 0..10 {
//         vec.push(i);
//     }
//     assert_eq!(vec.len(), 10);
//     assert_eq!(vec.capacity(), 10);
//
//     vec.push(11);
//     assert_eq!(vec.len(), 11);
//     assert!(vec.capacity() >= 11);
//
//     let mut vec = Vec::with_capacity(100);
//     for i in 0..100 {
//         vec.push(i);
//     }
//     assert_eq!(vec.len(), 100);
//     assert_eq!(vec.capacity(), 100);
//
//     println!("Success!");
// }

// fn main() {
//     // will not work
//     let v = vec![1, 2.0, 3];
// }

// #[derive(Debug, PartialEq)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }
// fn main() {
//     let v = vec![
//         IpAddr::V4("127.0.0.1".to_string()),
//         IpAddr::V6("::1".to_string()),
//     ];
//
//     assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
//     assert_eq!(v[1], IpAddr::V6("::1".to_string()));
//
//     println!("Success!");
// }