use std::io;

fn main(){
    // Array Type
    
    //declaration of the array

    // let arr = [1,2,3,4,5];

    // let months = ["January", "February", "March", "April", "May", "June", "July",
    //           "August", "September", "October", "November", "December"];

    // array declaration with type and sizez
    // let arr : [i32; 5] = [1,2,3,4,5];

    // arrays with size
    let arr = [3; 5];
    println!("Enter the index number");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];
    println!("{element}");

}