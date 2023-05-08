fn vectors() {
    let v: Vec<i32, 3> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(5);
    v.push(6);
    v.push(7);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element. "),
    }

    for i in &v {
        println!("{}", i)
    }
    {
        let mut v = vec![100, 32, 57];

        for i in &mut v{
            //(*) is a deference operator, is used to get the value of i
            //before we can use the += operator.
            *i += 50;
        }
    }
    {
        enum SpreadSheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
        let row = vec![
            SpreadSheetCell::Int((3)),
            SpreadSheetCell::Float((10.12)),
            SpreadSheetCell::Text((String::from("blue"))),
        ];
    }
}
