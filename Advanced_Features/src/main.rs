use std::slice;
use std::ops::Add;
use std::fmt;
use std::io::Error;

//extern block

extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    {
        //unsafe Rust;

        let mut num = 5;
        //raw pointers from references

        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        let address = 0x012345usize;
        let r = address as *const i32;

        //we can create raw pointers in safe rust,
        //but we can't deference raw pointers and read
        //the data being pointed to.

        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }

        unsafe fn dangerous() {}
        //call to unsafe function requires unsafe function or block
        unsafe {
            dangerous();
        }

        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        let (a, b) = r.split_at_mut(3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    
        fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = slice.len();
            let ptr = slice.as_mut_ptr();

            assert!(mid <= len);

            unsafe {
                (slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
            }
        }

        unsafe{
            println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    {
        //Static variables
        //the name of static variables must be in SCREAMING_SNAKE_CASE
        //

        static HELLO_WORLD: &str = "Hello, world!";

        println!("name id: {}", HELLO_WORLD);

        static mut COUNTER: u32 = 0;

        fn add_to_count(inc: u32) {
            unsafe {
                COUNTER += inc.
            }
        }

        add_to_count(3);

        unsafe {
            println!("COUNTER: {}", COUNTER);
        }

    }
}

    {
        //traits
        pub trait Iterator {
            type Item;

            fn next(&mut self) -> Option<Self::Item>;
        }

        impl Iterator for Counter {
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> {

            }
        }

        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl Add for Point {
            type Output = Point;

            fn add(self, other: Point) -> Point {
                Point {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        assert_eq!(Point {x: 1, y: 0} + Point {x: 2, y: 3},
                    Point {x: 3, y: 3});

        trait Add<RHS=Self> {
            type Output;

            fn add(self, rhs:RHS) -> Self::Output;
        }

        struct Millimeters(u32);
        struct Meters(u32);

        impl Add<Meters> for Millimeters {
            type Output = Millimeters;

            fn add(self, other:Meters) -> Millimeters {
                Millimeters(self.0 + (other.0 * 1000))
            }

        }
        trait Pilot {
            fn fly(&self);
        }

        trait Wizard {
            fn fly(&self);
        }

        struct Human;

        impl Pilot for Human {
            fn fly(&self) {
                println!("This is your captain speaking.");
            }
        }

        impl Wizard for Human {
            fn fly(&self) {
                prinln!("Up!");
            }
        }

        impl Human {
            fn fly(&self) {
                prinln!("Up*waving arms furiously*");
            }
        }

        let person = Human;
        Pillot::fly(&person);
        Wizard::fly(&person);
        person.fly();

        trait Animal {
            fn baby_name() -> String;
        }

        struct Dog;

        impl Dog {
            fn baby_name() -> String {
                String::from("Spot")
            }
        }

        impl Animal for Dog {
            fn baby_name() -> String {
                String::from("puppy")
            }
        }
        //print spot
        println!("A baby dog is called a {}", Dog::baby_name());

        //print puppy
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
        

    {
        
        struct Point {
            x: i32,
            y: i32,
        }

        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formater) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }

        trait OutlinePrint: fmt::Display {
            fn outline_print(&self) {
                let output = self.to_string();
                let len = output.len();
                println!("{}", "*".repeat(len + 4));
                println!("*{}*", " ".repeat(len + 2));
                println!("* {} *", output);
                println!("*{}*", " ".repeat(len + 2));
                println!("{}", "*".repeat(len + 4));
            }
        }

        struct Wrapper(Vec<String>);

        impl fmt::Display for Wrapper {
            fn fmt(&self, f: &mut fmt::Formater) -> fmt::Result {
                write!(f, "[{}]", self.0.join(","))
            }
        }

        let w = Wrapper(vec![String::from("hello"), String::from("world")]);

        println!("w {}", w);

    }
    
    }

    {
        //Advanced types

        type Kilometers = i32;

        let x: i32 = 5;
        let y: Kilometers = 5;

        println!("x + y = {}", x + y);

        type Result<T> = Result<T, std::io::Error>;

        pub trait Write {
            fn write(&mut self, buf: &[u8]) -> result<usize, Error>;
            fn flush(&mut self) -> Result<(), Error>;

            fn write_all(&mut self, buf: &[u8]) -> Result<()>;
            fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;

        }

        //never types are used in fuctions that never return
        fn bar() -> ! {
            //snip
        }
        //The golden rule of dynamically sized types is that
        //we must always put values of dynamically sized types
        //behind a pointer of some kind.

        fn generic<T>(t: T) {
            //snip
        }
        //rust implicitly adds a bound on Sized to every generic function like this:
        fn generic2<t: Sized>(t: T) {
            //snip
        }
        /*by default, generic functions will work only with types that have
        a know size at compile time
        However, you can use the following special syntax to
        relax this restriction:*/
        fn generic3<T: ?Sized>(t: &T) {
            // snip
        }

        fn add_one(x: i32) -> i32 {
            x + 1
        }

        fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
            f(arg) + f(arg)
        }

        let answer = do_twice(add_one, 5);

        println!("The answer is: {}", answer);

        let list_of_numbers = vec![1, 2, 3];
        let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();

        enum Status {
            Value(u32),
            Stop,
        }

        let list_of_statuses: Vec<Status> = 
        (0u32..20)
        .map(Status::Value)
        .collect();

        fn returns_closures() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }

    }

    {
        //macros

        #[macro_export]
        macro_rules! vec {
            ( $( $x:expr ),* ) => {
                {
                    let mut temp_vec = Vec::new();
                    $(
                        temp_vec.push($x);
                    )*
                    temp_vec
                }
            };
        }


    }

}
