fn main(){
    // Tuple Data Type

    let tup: (u8, i8, f32, char, bool) = (12,-13,14.10, 'A', true);

    let (a,b,c,d,e) = tup;

    let _first:u8 = tup.0;
    println!("{a}, {b}, {c}, {d} , {e}");
    println!("{}, {}, {}, {} , {}", tup.0, tup.1, tup.2, tup.3, tup.4);
}