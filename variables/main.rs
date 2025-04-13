const SUM:i8 = 40;

fn main(){

    // variables and mutability
    let mut x = 10;
    println!("The value of x is {x}");
    x = 6;
    println!("Now, The value of x is {x}");

    // constants
    const THREE_HOURS_IN_SECOND:i32 = 3*60*60;
    println!("{THREE_HOURS_IN_SECOND}");
    println!("sum is: {SUM}");

    // shadowing 
    let a = 10;
    let a = a+1;

    {
        let a = a*5;
        println!("The value of a in the inner scope is: {a}");
    }

    println!("The value of a is : {a}");

    let spaces = "   ";
    let spaces = spaces.len();
    
    println!("{spaces}");
}