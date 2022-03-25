pub fn run(){
    let mut count = 0;
    while count<=10 {
        if count%2==0{
            println!("Numero Even!")
        }
        else{
            println!("Numero ODD!")
        }
        count+=1;
    }
    //FOR RANGE
    for i in 0..10{
        println!("Number : {}", i);
    }
}