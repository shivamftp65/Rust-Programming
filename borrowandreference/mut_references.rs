fn main(){
    let mut s1 = String::from("hello");
    // 1. can have only one mutable reference
    // let r1 = &mut s1;
    // let r2 = &mut s1;
    // println!("{}, {}", r1, r2);
    // Error: cannot borrow `s1` as mutable more than once at a time

    // 2. can have any number of immutable references
    // 3. if there is a mutable refernece then can't have immutable refernce either
    // let r1 = &s1;
    // let r2 = &mut s1; // Error : cannot borrow `s1` as mutable because it is also borrowed as immutable
    // println!("{r1} {r2}");


    // this will also posible
    // {
    //     let r1 = &mut s1;
    // }

    // let r2 = &mut s1;

    // and this too

    let r1 = &s1;
    let r2 = &s1;
    println!("{r1} {r2}");

    let r3 = &mut s1;
    println!("{r3}");
}