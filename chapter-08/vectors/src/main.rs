fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100]; // will cause a panic reference and crash the program
    let does_not_exist = v.get(100); // retrns a None if the index does not exist so wont crash the program

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57]; // mutable
    for i in &mut v {
        *i += 50;
    }

    enum SpreadsheetCell { // sotring multiple types in a vector using enum
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];
}
