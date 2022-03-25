pub fn run(){
    //we use ref pointers because when non primitive values are assigned to another value the previous one disappears
     let vec1 : Vec<i32> = vec![1, 2, 3];
     let vec2 = &vec1;
     println!("{:?}", (&vec1, vec2));
}