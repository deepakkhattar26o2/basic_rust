// struct Color{
//     red: u8,
//     green: u8,
//     blue: u8
// }
struct Person{
    first_name : String,
    last_name : String
}
//constructor
impl Person{
    fn new(first: &str, last:&str)->Person{
        Person{
            first_name : first.to_string(),
            last_name: last.to_string()
        }
    }
    fn full_name(&self)->String{
        format!("{} {}", self.first_name, self.last_name)
    }
}
pub fn run(){
    // let obj = Color{
    //     red: 255,
    //     green: 0,
    //     blue: 0
    // };
    // println!("{} {} {}", obj.red, obj.green, obj.blue)
    let obj = Person::new("Deepak", "Khattar");
    println!(" First Name : {:?}, Last Name : {:?}", obj.first_name, obj.last_name);
    println!("Full Name : {}", obj.full_name());
}