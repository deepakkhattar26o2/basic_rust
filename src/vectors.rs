pub fn run(){
    let mut numeros : Vec<i32> = vec![1,2,3];
    println!("{:?}", numeros);
    //Adding an element to last index
    numeros.push(4);
    numeros.push(5);
    println!("New Vector: {:?}", numeros);
    //removing last elements
    numeros.pop();
    //Iterating through the vector
    for i in numeros.iter_mut(){
        //to mutate an element add asterisk
        *i *=2;
        println!("Number : {}", i)
    }
    
}