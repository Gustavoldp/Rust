use std::fml::Display;

fn longest(x: &str, y: &str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main(){
    {
    let r;

    {
        let x = 5;
        r = &x;
    }//x 'dies' here
    //r is null so it wont compile
    println!("r: {}". r);
    }

   {
     let r;

    let x = 5;
    r = &x;

    println!("r: {}", r);
    
    }

    {
        let string = String::from("long string is long");
        
        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is: {}", result);
        }

    }

    {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        let novel = String::from("Call me Ismael. Some years ago...");
        let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");

        let i = ImportantExcerpt {part: first_sentence};

        impl<'a> ImportantExcerpt<'a> {
            fn level(&self) -> i32 {
                3
            }
        }

        impl<'a> ImportantExcerpt<'a> {
            fn announce_and_return_part(&self, announcement: &str) -> &str {
                println!("Attention please: {}", announcement);
                self.part
            }
        }

        //static lifetime will life as long as the program
        let s: &'static str = "I have a static lifetime";

        fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display{
            println!("Announcement! {}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
    }
}