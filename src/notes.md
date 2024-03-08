# Notes

## Numbers - Integer Types
**Signed integer:** Can represent both positive and negative integers<br>
**Unsigned integer:** Always positive integers

| Length  | Signed | Unsigned |
|---------|--------|----------|
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

*Note: "arch" stands for architecture; this depends on your machine

## Numbers - Default Types
Integers: i32
Floats: f64

## Numbers - Binary Number Systems
42

4       2
10^1    10^0

(4&*10^1) + (2*10^0)
    = (4*10) + (2*1)
    = 40 + 2 = 42

## Numbers - Range of 8-bit integers
Smallest possible 8-bit integer (unsigned): 0
Largest possible 8-bit integer (unsigned): 255

## Numbers - Range of 16-bit integers
Smallest possible 16-bit integer (unsigned): 0
Largest possible 16-bit integer (unsigned): 65,535

## Numbers - Signed Integers
Signed integers use a process called "Two's complement"

42
- Takes binary
- Inverts each 0 and 1 in binary
- Add 1 bit
== -42


## Numbers - usize and isize
- Architecture dependent
- On 32-bit arch: 32-bit
- On 64-bit arch: 64-bit
- Pointer sized integer type, matches size of a *word* in given platform

## Numbers - What is a word?
- Process does NOT read 1 byte at a time from memory, reads 1 word at a time
- In a 32-bit processor it can access 4 bytes (32 bits) at a time
- In a 64-bit processor it can access 8 bytes (64 bits) at a time

In a 32-bit arch, the size of a word is 4 bytes, which means the processor can access 4 bytes at a time

`usize` gives you the guarantee to be always big enough to hold any pointer or any offset in a data structure

## Numbers - Floating Point
f32: size of 32 bits
f64: size of 64-bits

Representation according to IEEE-754 specification

---

## Ownership
- Rust ownership system is unique and sets it apart from other programming langs
- Set of rules that govern memory management
- Rules are enforced at compile time
- If any of the rules are violated, the program won't compile

### Three rules of ownership
1. Each value in Rust has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

**Owner:** The owner of a value is the variable or data structure that holds it and is
responsible for allocating and freeing memory used to store that data.

## Scope
- Range within a program for which an item is valid
- **Global scope:** Accessible throughout the entire program
- **Local scope:** Accessible ONLY within particular function or block of code

---

## Memory
- Component in a computer to store data and instructions for the processor to execute
- Random Access Memory (RAM) is volatile, when power turned off all contents are lost.
- Two types of regions in RAM used by a program at runtime: Stack memory and heap memory

## Stack Memory
- Last in, first out
- All data stored on the stack must have a known, fixed size (like ints, floats, chars, bools, etc)
- Pushing to the stack is faster than allocating on the heap, because the location for new data is
always at the top of the stack
- Types of unknown size will get allocated to the heap and a pointer to the value is pushed to the
stack, because a pointer is a fixed size (usize)

## Heap Memory
- Data of no known, fixed size belongs on the heap
- Allocating data on the heap will return a pointer (an address to a location where data has been allocated)
- Allocating on the heap is slower than pushing to stack
- Accessing data on the heap is also slower, as it has to be accessed using a pointer which points to an address

---

## The String Type
- All types covered so far were fixed size
- String is mutable
- String size can change at runtime
- String stored on the stack with a pointer to the heap
- Value of String is stored on the heap

`let s1 = String::from("hello");`

---

## Copy vs. Move
- Scalar values with fixed sizes (all types we covered at the beginning) will automatically get copied in the stack,
copying here is cheap
- Dynamically sized data won't get copied, but moved, copying would be too expensive

**Example**<br>
```
let x = 5;
let y = x;
```
`y` has the same value as `x`, but the value is stored at a different location

## Ownership and Functions
**Example**<br>
```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_pointer(&s);              // the '&' uses the pointer of s, not the actual var itself
                                    // meaning you will be able to use it after using this function
    
    takes_ownership(s);             // s's value moves into the function and so is no longer valid here
    
    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function, but i32 is copy, so it's
                                    // okay to still use x afterward
}

fn takes_pointer(some_string: &String) {
    printls!("{}", some_string);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
}                                         // here, some_string goes out of scope and `drop`
                                          // is called. the backing memory is freed

fn makes_copy(some_integer: i32) {        // some_integer comes into scope
    println!("{}", some_integer);
}                                         // here, some_integer goes out of scope; nothing special happens
```
```rust
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
```

## Preventing issues
- Ownership prevents memory safety issues:
  - Dangling pointers
  - Double-free
    - Trying to free memory that has already been freed
  - Memory leaks
    - Not freeing memory that should have been freed

## Borrowing
- Way of temporarily accessing data without taking ownership of it
- When borrowing, you're taking a reference (pointer) to the data, not the data itself
- Prevention of dangling pointers and data races
- Data can be borrowed immutabily and mutably
- There are certain rules when borrowing which we have to comply with, otherwise the program won't compile

## Rules of References
1. At any given time, you can have either one mutable reference or any number of immutable references
2. References must always be valid

## String vs. &str
- A String is a heap-allocated string type that owns its contents and is mutable
- A &str is an immutable sequence of UTF-8 bytes in memory, it does not own the underlying data and is immutable
- Think of &str as a view on a sequence of characters (stored as UTF-8 bytes) in memory
- Use &str if you just want to view a string
- &str is more lightweight and efficient than String
- Use String if you need to own the data and be able to mutate it

## String Literal
- A string literal is a sequence of characters enclosed in double quotes
- Fixed size, compile-time known sequence of UTF-8 bytes
- The type is `&'static str` which indicated the data is stored in static storage, meaning it is valid throughout
the entire lifetime of the program
- The data is hardcoded into the executable and stored in read-only memory, meaning they are immutable

---

## Arrays
- Fixed-size collection of elements of the same data type stored as a contiguous block in stack memory
- Signature of array is [T, Length] which indicates that the length is fixed at compile time
- Arrays can neither grow nor shrink, they must retain their size

---

## Slice
- Reference to contiguous sequence of elements in a collection
- Provide a way to borrow part of a collection without taking ownership of the entire collection
- Can be created from arrays, vectors, Strings, and other collections implements the De`ref` trait

**Example**<br>
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    
    assert_eq!(slice, &[2, 3]);
}
```

## Tuples
- Way to store related pieces of information in a single variable
- Collection of values of different types grouped together as a single compound value (tye composed of other types)
- Stored as a fixed-size contiguous block of memory on the stack
- Signature is (T, T, T...)

---

## Struct
- Compound type allowing to group together values of different types into a named data structure
- Similar to tuples, but each value has a name so values can be accessed through this name
- Have to be instanced with data, think of it like the struct is the template for the instances you create form it

**Example**<br>
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
```
```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };
}
```

## Tuple Structs
- Like normal structs but using tuple-like syntax for defining their fields
- Basically a named tuple
- Initiated by parenthesis instead of curly braces
- Accessed through point notation

**Example**<br>
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

## Unit-like Structs
- Structs without any field(s)
- Used when working with traits (covered later)
- Doesn't store any data

---

## Enum
- Way of defining a type with only one of a possible set of values
- We can only access one variant of an enum at a time
- Can hold additional information using tuples
- Especially useful when in `match` statements

**Example**<br>
```rust
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::v4(String::from("127.0.0.1"));
    
    let loopback = IpAddr::V6(String::from("::1"));
}
```

## The Option Enum
- Option is an enum that represents a value that may or may not be present
- Known in other languages as "null" referring to the absence of a value
- Used to handle cases where a function or method might fail to return a value

---

## Flow Control
- Normal flow of a program: Top to bottom, line by line
- Concept that refers to the ability to control the order in which statements or instructions are executed in a program
- Allows to specify which instructions should be executed under which conditions and in what order
- Conditionals:
  - if/else
  - match
- Loops
  - for/while/loop
  - continue/break

---
## Pattern Match
- Powerful construct that allows to compare a value against set of patterns, then execute different code based on 
which pattern matches
- Patterns can be made up of literal values, variable names, wildcards, etc...
- In match, all possible cases must be handled, enforced by the compiler
```rust
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}
```

Here, we have an enum Coin which holds different denominations of US coins.<br><br>

`value_in_cents()` takes as argument a Coin (which can only hold field of the enum) and checks, which field of the enum
has been matched. Then returns the right amount as u8.<br><br>

**if let**
```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}
```
becomes
```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```

---

## Methods
- Function that is associated with a particular type of struct
- Takes parameters and returns a value, but defined as a member of a struct or enum
- Called using dor notation (like accessing members of a struct)
- Implemented through an "impl" block

**Example**
```rust
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}
impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
}
```

## Associated Functions
- Function that is associated with a struct or an enum, but doesn't take an instance as its first paramater
- Called using the name of the type, not an instance of it
- Often used as constructors for a struct or enum

**Example**
```rust
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}
impl Rectangle {
  fn new(width: u32, height: u32) -> Rectangle {
    Rectangle {
      width, height
    }
  }
}
fn main() {
  let rec1 = Rectangle::new(5, 10);
  
  println!("Rectangle: {:?}", rec1);
}
```

---

## Generics
- Placeholders for concrete types
- Enables writing more reusable and flexible code
- Avoids having duplicate code for different types
- Zero cost abstraction, Rust compiler will at compile time fill out generics with concrete types

## Const Generics
- Type parameter that represents a compile-time constant value
- Allows to write generic code that operates on values that are known at compile time
- Used for array sizes, bit widths and other constants

---

## Traits
- Set of methods that can implemented for multiple types in order to provide common functionality and behaviour
between them
- Traits consist of method signatures only, which then have to be implemented by the target type
- Similar to "classes" in other languages, not quite the same though
- Defines shared behavior in an abstract way

**Example**
```rust
trait Animal {
    fn sound(&self) -> String;
}
    struct Sheep;
    struct Cow;
    
    impl Animal for Sheep {
        fn sound(&self) -> String {
          String::from("Baah");
        }
    }
    
    impl Animal for Cow {
        fn sound(&self) -> String {
          String::from("Moo");
        }
    }
}
```

## Derivable Traits
- Trait that can be automatically implemented for a struct or an enum by the Rust compiler
- Called "derivable" because they can be derived automatically
- Most common derivable traits:
  - Debug: allowing to output content via "{:?}"
  - Clone: Enables type to be duplicated with "clone()"
  - Copy: Enables type to be copied implicitly, without requiring explicit "clone()" method
  - PartialEq: Enables comparison

## Traits as Parameters
Traits can be used as parameters for functions

**Example**
```rust
pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}
```
The function `notify()` takes as argument any type that has implemented the Summary trait

## Trait Bounds
Similar to example using `impl Summary`, but more verbose. Trait bounds are declared like generics,
after the name of the function. Use trait bounds if you have lots of parameters to avoid this:
```rust
// not using trait bounds
pub fn notify(item1: &impl Summary, item2: &impl Summary) { }
// using trait bounds
pub fn notify<T: Summary>(item1: &T, item2: &T) { }
```

## Where Clauses
If you have a function that makes heavy 8use of trait bounds, we can use a where clause to make the code clear:
```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where 
    T: Display + Clone,
    U: Clone + Debug
```

## Return Types that Implement Traits
Here we have a trait `Animal` which is implemented for two structs, `Dog` and `Cat`.
The two functions return a struct, either `Dog` or `Cat`, that implements the `Animal` trait.
```rust
trait Animal { }

struct Dog;
struct Cat;

impl Animal for Dog { }
impl Animal for Cat { }

fn return_dog() -> impl Animal {
  Dog { }
}

fn return_cat() -> impl Animal {
  Cat {}
}

fn main() {
  return_dog();
  return_cat();
}
```

## Trait Objects
- Using `impl Trait` doesn't work when returning multiple types
- Different implementations of a trait probably use different amounts of memory, but sizes of types must be known at 
compile time
- In this case, trait objects can be used
- A trait object is essentially a pointer to any type that implements the given trait, where the precise type can only 
be known at runtime

## Dynamic Trait Objects
Here we have a function which returns a type that implements the `Animal` trait. This could be `Dog` or `Cat`. As
The trait object is behind a pointer, the size is known at compile time, which is usize (size of a pointer).
<br>
This allows for more flexible code as the exact return type doesn't have to be known at compile time as long as
the size is fixed.
```rust
trait Animal { }

struct Dog;
struct Cat;

impl Animal for Dog { }
impl Animal for Cat { }

fn return_animal(s: &str) -> &dyn Animal {
  match s {
    "dog" => &Dog {},
    "cat" => &Cat {},
    _ => panic!(),
  }
}

fn main() {
  let animal1 = return_animal("cat");
  let animal2 = return_animal("dog");
}
```

## Static Dispatch
- Resolves method calls at compile time
- Compiler generates function code for each concrete type that implements trait
- Calls appropriate function based on concrete types
- Faster and more efficient than dynamic dispatch, but doesn't provide great flexibility
<br>
The compiler generates methods for each concrete type (Dog, Cat) and the called method say_hi() can be resolved 
because it is known at compile time which method for which type has to be resolved.
<br>
Note: The output might not actually like that because compiler applies optimizations.

```rust
trait Animal {
  fn say_hi(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
  fn say_hi(&self) {
    println!("Woff");
  }
}
impl Animal for Cat {
  fn say_hi(&self) {
    println!("Meow");
  }
}

fn main() {
  let dog = Dog;
  let cat = Cat;
  
  dog.say_hi();
  cat.say_hi();
}
```
would become
```rust
struct Dog;
struct Cat;

impl Animal for Dog {
  fn say_hi(&self) {
    println!("Woff");
  }
}
impl Animal for Cat {
  fn say_hi(&self) {
    println!("Meow");
  }
}

fn main() {
  let dog = Dog;
  let cat = Cat;
  
  dog.say_hi();
  cat.say_hi();
}
```

## Dynamic Dispatch
- Specific methods to be called is determined at runtime
- Works by creating a reference or smart pointer to a trait object using `&dyn` or `Box<dyn >`
- When trait object is created, compiler will build a vtable for that trait
- vtable is a table that contains a pointer to the implementation of each method in the trait for the specific type of 
the object that the reference points to
- Compiler will do a lookup in a vtable to determine which method should be called for which type that implements
the given trait
- This lookup will cause overhead but allows for more flexible code

**Example**
```rust
trait Animal {
  fn noise(&self);
}

struct Cat;
struct Dog;

impl Animal for Cat {
  fn noise(&self) {
    println!("Meow");
  }
  
  impl Animal for Dog {
    fn noise(&self) {
      println!("Woof");
    }
  }
}
```
- We have an Animal trait and two structs that implement this trait
- We have a function that takes random number and returns a type that implements the animal trait (either cat or dog)
- Compiler cannot know the size at compile time, as one type might bigger than the other
- Then noise() method gets called on returned type that implements Animal trait, again compiler cannot know this, 
this resolves at runtime
```rust
fn random_animal(random_number: u8) -> Box<dyn Animal> {
  if random_number < 10 {
    Box::new(Cat { })
  } else {
    Box::new(Dog { })
  }
  
  fn main() {
    let random_number = 5;
    let animal = random_animal(random_number);
    animal.noise();
  }
}
```

## Box
- Smart pointer that allows to store data on the heap rather than the stack
- Use Box when you have  a type whose size can't be known at compile time
- Returns a pointer to the data stored on the heap

## & vs Box
- **Memory:** Box allocates data on heap and owns it, also responsible for deallocating when value goes out of scope, 
reference only points to a value already in memory
- **Lifetime:** Box can be passed across scope, reference has limited lifetime
- Box can be cloned, reference not
- Box can be used in pattern matching

---

## Associated Types
- Allow to specify a type that is associated with the trait
- When implementing the trait for a specific type we have to specific the concrete type
- Basically a type placeholder that the trait methods can use in their signature
- Similar to generic types but are more flexible because they allow a trait to have different associated types for
different implementing types
<br>

Below we define a trait that has an associated type and method that return a value of this type.
**Example 1**
```rust
trait MyTrait {
  type MyType;
  
  fn get_my_type(&self) -> Self::MyType;
}
```
When implementing the trait for a specifi type (MyStruct), then we have to give the associated type MyType a concrete 
type, in this case i32.
**Example 2**
```rust
struct MyStruct { }

impl MyTrait for MyStruct {
  type MyType = i32;
  
  fn get_my_type(&self) -> Self::MyType {
    return 42;
  }
}
```

---

## The String Type
A string consists of 3 things:
1. Pointer to data stored on the heap
2. Data size in bytes
3. Total amount of memory received from the allocator
```rust
let s1 = String::from("hello");
```
| name     | value         |
|----------|---------------|
| ptr      | see ptr table |
| len      | 5             |
| capacity | 5             |

**Pointer table**<br>

| index | value |
|-------|-------|
| 0     | h     |
| 1     | e     |
| 2     | l     |
| 3     | l     |
| 4     | o     |

---

## Vectors
- Like arrays, but dynamically sized, meaning can grow and shrink
- Allocated on the heap as a contiguous block of memory
- All elements must have the same type
- Special macro: vec!