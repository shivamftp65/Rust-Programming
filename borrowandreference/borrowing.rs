fn main(){
    let mut s1 = String::from("hello");
    let lenght = lenght_of_string(&s1);
    println!("{lenght}");

    // what if you want to update the string ?
    update_str(&mut s1);

    println!("{s1}");
}

fn lenght_of_string(str : &String) -> usize{
    str.len()
}

fn update_str(str: &mut String){
    str.push_str(", World");
}