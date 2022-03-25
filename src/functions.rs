pub fn run(){
    greet("Deepak");
    println!("{}",sqr(5));
    println!("{:?}", grt(41, 33))
}
fn greet(name: &str){
    println!("Hello, {}", name)
}
fn sqr(i1: i32)->i32{
    i1*i1
}
fn grt(i1: i64, i2: i64) ->bool{
    if i1>i2
    {
        return true;
    }
    return false;
}