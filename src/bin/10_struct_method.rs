#![allow(unused)]
#[derive(Debug)]

struct Rectangle{
    width: f32,
    height: f32,
}

impl Rectangle {
    // Associated function: Creates a new Rectangle with given width and height
    fn new(width: f32, height: f32) -> Self{
        Self {width, height}
    }

    // Associated function: returns a Rectangle with both width and height set to ZERO
    fn zero() -> Self {
        Self { width: 0.0, height: 0.0 }
    }

    // Method: calculates the area of the rectangle
    fn area(&self) -> f32 {
        self.width * self.height
    }

    // Method: resizes the rectangle to new dimensions
    fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
}
fn main(){
    // Creating a new rect with w = 3 and h = 4
    let mut rect = Rectangle::new(3.0, 4.0);
    
    println!("Original area: {}", rect.area()); // 12

    // Resize the rect to w = 5 and h = 6
    rect.resize(5.0, 6.0);

    println!("Resized area: {}", rect.area()); // 30

    // Creating a zero-sized rect (w = 0 and h = 0)
    let zero_rect = Rectangle::zero();

    println!("Zero Rectangle: {:?}", zero_rect); 
}