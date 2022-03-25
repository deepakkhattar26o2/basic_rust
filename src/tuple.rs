//Group Together values of different types, max 12 elements
pub fn run(){
    let tup : (&str, &str, i32) = ("Deepak", "Khattar", 19);
    println!("{:?}", tup);
    println!("one: {}, two: {}, three: {}", tup.0, tup.1, tup.2);
}