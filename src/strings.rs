//Primitve str => Immutable
//String => Dynamic , Heap Allocated
pub fn run() {
    let mut hello = String::from("Hello Baka no Baka"); //Dynamic String
    println!("{}", hello);
    //Get Length of String
    println!("Length : {}", hello.len());
    //pushing
    hello.push_str("hehehaha");
    //check if it's empty
    //hello.is_empty()
    //check for containing a word
    // hello.contains(a.contains("rock"))
    for word in hello.split(" "){
        println!("haha{} ", word);
    }
    println!("{}", hello);
}
