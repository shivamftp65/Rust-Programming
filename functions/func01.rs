fn main(){
    println!("This is main function");
    another_function();
    let sum:i8 = print_sum(10,16);

    println!("The sum is : {sum}");

    // expression
    let y = {
        let x = 7;
        x+7
    };

    println!("The value of y is: {y}");
}

fn another_function(){
    println!("THis is another function");
}

fn print_sum(x:i8, y:i8)-> i8{
    let a  = x+y;
    return a;
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}