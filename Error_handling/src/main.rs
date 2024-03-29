use std::io;
use std::fs::File;
use std::io::Read;

fn main() {
    //panic!("crash and burn");

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };

    {
        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => panic!("Problem opening the file: {:?}", other_error),
            },
        };

    }

    {
        //using unwrap multiple times can make finding the error a difficult task
        //since all functions that panic will print the same message.
        let f = File::open("hello.txt").unwrap();
        let g = File::open("hello.txt").expect("Failed to open hello world");
    }

    {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }

    }

    {
        fn read_username_from_file() -> Result<String, io::Error>{
            let mut f = File::open("hello.txt")?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        }
    }

    {
        fn read_username_from_file() -> Result<String, io::Error> {
            let mut s = String::new();

            File::open("hello.txt")?.read_to_string(&mut s)?;

            Ok(s)
        }
    }

    {
        fn read_username_from_file() -> Result<String, io::Error>{
            fs::read_to_string("hello.txt")
        }
    }

    {
        loop {
            let guess: i32 = match guess.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            };

            if guess < 1 || guess > 100{
                println!("The secret number will be between 1 and 100");
                continue;
            }
            match guess.cmp(&secret_number){
                // --snip--
            }

        }
    }

    {
        pub struct Guess {
            value: i32,
        }

        impl Guess {
            pub fn new(value: i32) -> Guess {
                if value < 1 || value > 100 {
                    panic!("Guess value must be between 1 and 100, got {}", value);
                }

                Guess {
                    value
                }
            }
            pub fn value(&self) -> i32 {
                self.value
            }
        }
    }

}
