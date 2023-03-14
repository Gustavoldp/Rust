fn main() {
    {
    //it workds, but the variables are not connected
    //area has two parameters, instead of one instance of a rectangle
    let width: u32 = 30;
    let height: u32 = 50;

    println!("The area of the retangle is {} square pixels.", 
    area(width, height));

    fn area(width: u32, height: u32) -> u32{
        width * height
    }
    }
    {
        //this way the function receaves a single instance.
        //it makes more sense.
        let rect:(u32, u32) = (30, 50);

        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1
        }

        println!("The area of the retangle is {} square pixels.",
        area(rect))
        
    }
    {
        //this way we know our structure
        let rect1: Rectangle = Rectangle{
            width: 30,
            height: 50,
        };

        println!("{:?}", rect1);

        //prettier
        println!("{:#?}", rect1);

        //Print the rectangle area
        rect1.area();

        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }
        {
        let rect1: Rectangle = Rectangle{
                width: 32,
                height: 60,
        };
        let rect2: Rectangle = Rectangle { width: 50, height: 75 };

        println!("{}",rect1.can_hold(&rect2));
        println!("{}", rect2.can_hold(&rect1));
        }
    }
}
#[derive(Debug)] //allows us to print the struct to help debugging
struct Rectangle{
    width: u32,
    height: u32,
}
impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}
