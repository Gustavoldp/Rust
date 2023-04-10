fn string(){
    let mut s = String::new();

    let data = "Initial data";

    let s = data.to_string();

    let s = "Initial Data".to_string();

    let s = String::from("initial content");

    {
        let hello = String::from("ϢϜϴϠϋϡϼδϟ΍");
        let hello = String::from("Dobrý den");
        let hello = String::from("Hello");
        let hello = String::from("ʭˣʬʝʕʹ");
        let hello = String::from("नमस्ते");
        let hello = String::from("こんにちは");
        let hello = String::from("안녕하세요");
        let hello = String::from("你好");
        let hello = String::from("Olá");
        let hello = String::from("Здравствуйте");
        let hello = String::from("Hola");
    }

    {
        let mut s = String::from("foo");
        s.push_str("bar");
    }

    {

        //push_str does not take ownership of s2, so we can print it after we push to s
        let mut s = String::from("foo");
        let s2 = "bar";
        s.push_str(s2);
        println!("s2 is {}", s2);
    }

    {
        let mut s = String::from("lo");
        s.push_str("l");
    }

    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("World");
        let s3 = s1 + &s2;
        //s1 can no longer be used since it's is used to append a reference of s2
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);
        //all variables can be used sinse the format! macro doesn't take ownership of them
    }

    {
        //iterates over the string and return a scalar of chars
        for c in "नमस्ते".chars() {
            println!("{}", c);
           }
           //itarates over the string and return the value of each of the 18 bytes in it
           for b in "नमस्ते".bytes() {
            println!("{}", b);
           }
        //getting grapheme clusters cannot be done with the standard library
    }

}