# Rust theoretical concepts

Rust bible: [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)

Rust by example: [https://rust-exercises.com/01_intro/00_welcome](https://rust-exercises.com/01_intro/00_welcome)

- Variable types explained
    - ***Primitive Types**:*
        - **Integer Types**: `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`.
            - Represent whole numbers, both signed (`i`) and unsigned (`u`) with various bit sizes. Different bit size depends on how big your number is. Default is assigned to 32.
        - **Floating-Point Types**: `f32`, `f64`.
            - Represent numbers with fractional parts of different bit sizes.
        - **Boolean Type**: `bool`.
            - Represents true or false values.
        - **Character Type**: `char`.
            - Represents a Unicode scalar value. A Unicode scalar value is any value in the Unicode codespace; that is, the range of integers from 0 to 0x10FFFF. 
            a `char` is a 4-byte Unicode scalar value, which means it can represent characters from the entire Unicode range.
    - ***Compound Types**:*
        - **Arrays**: `let arr: [i32; 5] = [1, 2, 3, 4, 5];`.
            - **Arrays**: Fixed-size collections of items of the same type.
        - **Tuples**: `let tup: (i32, f64, char) = (500, 6.4, 'a');`.
            - **Tuples**: Fixed-size collections of items of possibly different types.
    - ***References**:*
        - **Immutable References**: `&T`.
            - **Immutable References**: Allow borrowing data without allowing modification.
        - **Mutable References**: `&mut T`.
            - **Mutable References**: Allow borrowing data for modification, but with restrictions on aliasing.
    - ***Pointers**:*
        - **Raw Pointers**: `const T`, `mut T`.
            - **Raw Pointers**: Unmanaged pointers, can be dereferenced but don't have ownership or borrowing semantics  (in concept close to C and Cpp pointers).
    - ***Slices**:*
        - **Slices for Arrays**: `&[T]`.
            - **Slices for Arrays**: Provide a view into a contiguous sequence of elements in an array.
        - **Slices for Dynamic Data**: `&str`.
            - **Slices for Dynamic Data**: Represents a view into a UTF-8 encoded string.
    - ***Ownership Types**:*
        - **Owned Types**: `String`, `Vec<T>`, `Box<T>`.
            - **Owned Types**: Have unique ownership over their data, allowing mutation and deallocation. Meaning that they can be changed and size is variable.
        - **Rc (Reference Counting)**: `Rc<T>`.
            - **Rc (Reference Counting)**: Allows multiple ownership with reference counting, but not for mutable data.
        - **Arc (Atomic Reference Counting)**: `Arc<T>`.
            - **Arc (Atomic Reference Counting)**: Thread-safe version of `Rc`.
- Key and Borrowing + Borrow Checker
    
    ![Untitled](Rust%20theoretical%20concepts%2023a2785fab994b4db7508c6c19feb291/Untitled.png)
    
    ### Rules of Ownership
    
    Ownership is a set of rules that governs how a Rust program manages memory. The main rules are:
    
    1. **Each value in Rust has a variable that’s its owner.**
    2. **There can only be one owner at a time.**
    3. **When the owner goes out of scope, the value will be dropped.**
    
    ### Rules of Borrowing:
    
    1. At any given time, you can have either one mutable reference or any number of immutable references.
        1. `&var`:  value is borrowed and cannot be modified. 
        2. `&mut var:` value can be modified
    2. References must always be valid.
    
    ### Example of Ownership:
    
    ```rust
    fn main() {
        let s1 = String::from("hello"); // s1 is the owner of the String
        let s2 = s1; // ownership of the String is moved to s2
    
        // s1 is no longer valid here
        println!("{}", s2); // this is valid
        // println!("{}", s1); // this would cause a compile-time error
    } // s2 goes out of scope here and the String is dropped
    
    ```
    
    In this example, `s1` owns the `String` initially. When `s1` is assigned to `s2`, `s1` loses ownership, and `s2` becomes the new owner. Rust enforces this rule at compile time, ensuring memory safety.
    
    ### Borrowing
    
    Borrowing allows you to reference a value without taking ownership of it. Borrowing comes in two flavors: immutable borrowing and mutable borrowing.
    
    ### Immutable Borrowing:
    
    You can have multiple immutable references to a value, but you cannot have a mutable reference while immutable references exist.
    
    ```rust
    fn main() {
        let s1 = String::from("hello");
    
        let len = calculate_length(&s1); // borrow s1 immutably
    
        println!("The length of '{}' is {}.", s1, len); // s1 is still valid here
    }
    
    fn calculate_length(s: &String) -> usize {
        s.len() // use the borrowed value
    } // s goes out of scope here, but because it doesn't own the value, nothing happens
    
    ```
    
    ### Mutable Borrowing:
    
    You can have only one mutable reference to a value at a time. This ensures that you cannot have data races at compile time.
    
    ```rust
    fn main() {
        let mut s = String::from("hello");
    
        change(&mut s); // borrow s mutably
    
        println!("{}", s); // s is still valid here
    }
    
    fn change(s: &mut String) {
        s.push_str(", world"); // modify the borrowed value
    } // s goes out of scope here, but because it doesn't own the value, nothing happens
    
    ```
    
- Integer overflow and saturation arithmetic
    
    To deal with integer overflow there are two main ways. overflow-checks (a global option, see Profiles for more options) or using wrapping_/saturating_ methods. 
    
    ❗**Wrapping**: If you think of all the possible values for a given integer type as a circle, wrapping around means that when you reach the maximum value, you start again from the minimum value.
    
    ❗**Saturating:** if integer operation overflows/underflows, the maximum or minimum value is returned.  
    
- Profiles: Dev and release
    
    A [**profile**](https://doc.rust-lang.org/cargo/reference/profiles.html) is a set of configuration options that can be
    used to customize the way Rust code is compiled.
    
    Cargo provides two built-in profiles: `dev` and `release`.
    
    The `dev` profile is used every time you run `cargo build`, `cargo run` or `cargo test`. It's aimed at local
    development,
    therefore it sacrifices runtime performance in favor of faster compilation times and a better debugging experience.
    
    The `release` profile, instead, is optimized for runtime performance but incurs longer compilation times. You need
    to explicitly request via the `--release` flag—e.g. `cargo build --release` or `cargo run --release`.
    
    - Overflow checks
        
        Overflow occurs when the resulting number in a operation is greated than the allocated primitive (eg. allocated u8, but the result would be expressed by a u32).
        
        Rust lets you, the developer, choose which approach to use when an integer overflow occurs.
        The behaviour is controlled by the `overflow-checks` profile setting.
        
        If `overflow-checks` is set to `true`, Rust will **panic at runtime** when an integer operation overflows.
        If `overflow-checks` is set to `false`, Rust will **wrap around** when an integer operation overflows. This will escalate the resulting integer to the correct primitive.
        
        ![example of a dev profile in Cargo.toml file.](Rust%20theoretical%20concepts%2023a2785fab994b4db7508c6c19feb291/Untitled%201.png)
        
        example of a dev profile in Cargo.toml file.
        
        Here you can find more settings: [https://doc.rust-lang.org/cargo/reference/profiles.html](https://doc.rust-lang.org/cargo/reference/profiles.html)
        
- Type casting (type conversion)
    
    ```rust
    let a: u32 = 10;
    
    // Cast `a` into the `u64` type. This keeps the same decimal value as there are enough bits to represent it.
    let b = a as u64;
    ```
    
    ![Behavior observed when converting from larger to smaller type (ie. i16—> i8)](Rust%20theoretical%20concepts%2023a2785fab994b4db7508c6c19feb291/Untitled%202.png)
    
    Behavior observed when converting from larger to smaller type (ie. i16—> i8)
    
- Code sharing between files and visibility in Rust projects
    
    Functions are private (not visible) between files.  
    
    You can modify the default visibility of an entity using a **visibility modifier**.
    
    Some common visibility modifiers are:
    
    - `pub`: makes the entity **public**, i.e. accessible from outside the module where it's defined, potentially from
    other crates.
    - `pub(crate)`: makes the entity public within the same **crate**, but not outside of it.
    - `pub(super)`: makes the entity public within the parent module.
    - `pub(in path::to::module)`: makes the entity public within the specified module.
    
    You can use these modifiers on modules, structs, functions, fields, etc.
    For example:
    
    ```rust
    pub struct Configuration {
        pub(crate) version: u32,
        active: bool,
    }
    ```
    
- Encapsulation
    
    Allows to access struct variables by keeping them private. 
    
    This is achieved by writing specific methos on struct which are public.
    
    ```rust
    pub mod ticket {
        pub struct Ticket {
            title: String,
            description: String,
            status: String,
        }
    
        impl Ticket {
            pub fn new(title: String, description: String, status: String) -> Ticket {
                if title.is_empty() {
                    panic!("Title cannot be empty");
                }
                if title.len() > 50 {
                    panic!("Title cannot be longer than 50 bytes");
                }
                if description.is_empty() {
                    panic!("Description cannot be empty");
                }
                if description.len() > 500 {
                    panic!("Description cannot be longer than 500 bytes");
                }
                if status != "To-Do" && status != "In Progress" && status != "Done" {
                    panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
                }
            
                Ticket {
                    title,
                    description,
                    status,
                }
            }
            // Add three public methods to the `Ticket` struct. 
            //This allows to acces values by leaving the struct values private
            pub fn title(self) -> String{
                return self.title;
            }
    
            pub fn description(self)-> String{
                return self.description;
            }
    
            pub fn status(self)-> String{
                return  self.status;
            }
    
    ```