fn main() {
    println!("Hello, world!");
    let mut s = String::new(); // creating a string
    s.push_str("bar");

    let data = "initial contents";

    let s = data.to_string();

    // The method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents"); // similar to use .to string

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let emoji = "🦀";
    println!("{emoji}");
    println!("bytes: {}", emoji.len()); // 4
    println!("chars: {}", emoji.chars().count()); // 1

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); // works like println!

    for c in "Зд".chars() {
        // iterating over string
        println!("{c}");
    }

    for b in "Зд".bytes() {
        // return the 4 bytes that make up each string
        println!("{b}");
    }
}
