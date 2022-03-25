use std::mem;
pub fn run(){
    //Fixed Size Obvio
    let mut numeros : [i32;5] = [1,2,3,4,5];
    //it has to be exact that number of elements and the data type
    //Arrays are Stack Allocated
    println!("{:?}", numeros);
    numeros[1] = 20;
    println!("{:?}", numeros);
    //Number of Elements in the array
    println!("{}", numeros.len());
    //Memory Used by the array
    println!("Memory Used: {} Bytes", mem::size_of_val(&numeros));
}