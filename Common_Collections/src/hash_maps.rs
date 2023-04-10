use std::collections::HashMap;

fn hashMap(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = socres.get(&team_name);

    for(key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores_hashMap: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

    {
        let field_name = String::from("Favorite Color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();

        map.insert(field_name, field_value);
        //filed_name and filed_value are invalid at this point
        //String does not have the copy trait so map takes ownership
    }

    {
        let mut scores = HashMap::new();

        //overwriting a value
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 50);

        println("{:?}", scores);

        //verifying if the key already exists and if not it inserts
        //the parameter as the new value for this key
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(20);
    }

    {
        //updating based on the old value

        let text = "hello world wondeful world";

        let mut map = hashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
    }
}