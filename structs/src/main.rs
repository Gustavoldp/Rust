fn main() {
    {
        //structs
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someoneusername123"),
        active: true,
        sign_in_count: 1,
    };

    //mutable user instance
    let mut user2 = User {
        email: String::from("someoneelse@example.com"),
        username: String::from("someoneelseusername123"),
        active: true,
        sign_in_count: 2,
    };
    user2.email = String::from("anotheremail@example.com");

    let user3: User = User{
        email: String::from("anotheremail@example.com"),
        username: String::from("anotherusername567"),
        //Reciclyng user1's active and sign count.
        ..user1
    };

    fn build_user(email: String, username: String) -> User{
        User { 
            username: username,
            email: email,
            active: true,
            sign_in_count: 1,
        }
    }
    fn build_user_init_short_hand(email: String, username: String) -> User{
        User { 
            //no need to specify since the names are equal. Short Hand syntax
            username,
            email,
            active: true,
            sign_in_count: 1,
        }
    }
    }
    {
        //tuple structs
        //tuples suport diferent types.
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black: Color = Color(0,0,0);
        let origin: Point = Point(0,0,0);
    }
    
}