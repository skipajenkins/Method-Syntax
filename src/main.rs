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


