fn main(){
    let age = 11;

    if age >= 18 {
        println!("You are legal to vote");
    } else {
        println!("You are not elligble for vote");
    }

    let number = 3;

    if number % 4 == 0 {
        println!("Number is divisible by four");
    } else if number % 3 == 0 {
        println!("Number is divisible by three");
    } else if number % 2 == 0 {
        println!("Number is divisible by two");
    } else{
        println!("Number is divisible by some other number");
    }

    let condition = number % 4 == 0;
    let num1 = if condition {5} else {9};

    println!("{num1}");
}