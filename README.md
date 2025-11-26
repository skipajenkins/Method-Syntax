# 🦀 Method Syntax in Rust

This project explores method syntax in Rust — how to attach behavior directly to structs using impl blocks.
You’ll learn how methods work, how they differ from normal functions, how to use &self, &mut self, and how to define associated functions like constructors.

This continues your Rust learning journey through ownership, references, borrowing, slicing, and structs — now moving into object-oriented style behavior written in Rust’s safe and ergonomic way.

---

## ⚙️ Setting Up the Environment

Before running this project, ensure Rust and Cargo are installed.

✔️ Step 1: Check Installation
```bash
rustc --version
cargo --version
```

If not installed, run:
```bash
curl https://sh.rustup.rs -sSf | sh
```

Then verify the installation again.

### 📁 Step 2: Create the Project
```bash
cargo new Method-Syntax
cd Method-Syntax
```

Replace the contents of src/main.rs with the code below.

---

## 📜 Rust Code
```bash
#[derive(Debug)]

struct Rectangle {
    width:u32,
    height:u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // fn can_hold(&self,other: &Rectangle) -> bool{
    //    self.width > other.width && self.height > other.height 
    // }
    fn square(size:u32) -> Self{
        Self{
            width: size,
            height: size,
        }
    }
}
impl Rectangle {
    fn can_hold(&self,other: &Rectangle) -> bool{
       self.width > other.width && self.height > other.height 
    }
}
//We can give the method the name of the field of the struct
// impl Rectangle {
//     fn width (&self) -> bool{
//         self.width > 0
//     }
// }

fn main (){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle{
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle{
        width: 60,
        height: 45,
    };
    let sq1 = Rectangle::square(4);

//     if rect1.width(){
//         println!("The rectangle has a nonzero width,it is {}.", rect1.width);
//     }
// }
    
    println!(
        "The area of the rectangle is {} square pixels.",
         rect1.area()
    );

    println!(
        "Can rect1 hold rect2? {}", rect1.can_hold(&rect2)
    );

    println!(
        "Can rect1 hold rect3? {}", rect1.can_hold(&rect3)
    );

    println!(
        "The area of the square is {} square pixels.", sq1.area() 
    );

    
   }
//    println!("The area of the rectangle is {} square pixels.",Rectangle{
//         width: 30,
//         height: 90,
//     }.area()
// ); THIS IS DUMB AND LONG AS FUCK. THIS APPROACH IS NOT NEEDED.
    
//}
```

---

## 🧠 Key Concepts Learned
Concept	Explanation
Methods	Functions defined inside an impl block that operate on a struct.
&self vs &mut self vs self	Determines how the method accesses or owns the struct.
Associated Functions	Functions in impl without self. Often used as constructors (Rectangle::square).
Chaining Behavior With Types	Methods allow behavior to be grouped logically with data.
Multiple impl Blocks	Rust allows splitting implementations to organize code cleanly.
### ▶️ Step 3: Build & Run
Build the project:
```bash
cargo build
```
Run the project:
```bash
cargo run
```
### 🧩 Example Output
The area of the rectangle is 1500 square pixels.
Can rect1 hold rect2? true
Can rect1 hold rect3? false
The area of the square is 16 square pixels.

### 🔍 How It Works
✔️ Method Calls
```bash
rect1.area()
```

is syntactic sugar for:
```bash
Rectangle::area(&rect1)
```
✔️ Borrowing with &self

Methods automatically treat self as the first parameter.
&self means "borrow this struct immutably."

✔️ Associated Function
```bash
Rectangle::square(4)
```

Creates a new instance without requiring an existing Rectangle.

✔️ Method Logic

The can_hold method compares two rectangles logically:
```bash
self.width > other.width && self.height > other.height
```

---

## 🎯 Learning Objectives

By completing this project, you practiced:

Creating methods using impl

Using self, &self, and referencing other struct instances

Splitting multiple impl blocks

Returning Self from an associated function

Understanding how Rust organizes code around data and behavior

---


## 🚀 Future Enhancements

Add a method for scaling rectangles

Implement method chaining (returning Self)

Add validation (no zero-sized rectangles)

Introduce traits and implement methods across multiple types

Write unit tests for area() and can_hold()

---

## 🦀 Built With

Rust

Cargo

---

## 📄 License

This project is licensed under the MIT License.
