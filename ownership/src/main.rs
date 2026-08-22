fn main() {
    let s = String::from("hello");
    eat(s);
    println!("{s}");
}

fn eat(text: String) {
    println!("食べた: {text}");
}