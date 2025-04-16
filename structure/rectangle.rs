// fn main(){
//     let width = 50;
//     let height = 40;

//     println!("The area of rectangle is:{}", area_reactangle(width, height));
// }

// fn area_reactangle(width:u32, height:u32) -> u32{
//     width*height
// }

// more readable way to do this is by using tuples

// fn main(){
//     let rect1 = (50, 40);
//     println!("The area of rectangle is:{}", area_reactangle(rect1));
// }

// fn area_reactangle(dimenstions:(u32, u32)) -> u32{
//     dimenstions.0 * dimenstions.1
// }

// Refactoring with Structs: Adding More Meaning
#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32
}

fn main(){
    let rect1 = Rectangle{
        width : 50,
        height : 40
    };

    println!("The area of rectangle is:{}", area_reactangle(&rect1));

    // try to print the reactangle
    // `Rectangle` doesn't implement `std::fmt::Display`
    // `Rectangle` doesn't implement `Debug`
    dbg!(&rect1);
    println!("{:?} or {:#?}", rect1, rect1);  
}

fn area_reactangle(rectangle : &Rectangle ) -> u32{
    rectangle.width * rectangle.height
}