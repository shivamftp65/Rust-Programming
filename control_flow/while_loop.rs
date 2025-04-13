fn main(){
    let mut count = 0;
    while count != 5{
        println!("count : {count}");
        count += 1;
    }

    let arr = [1,2,3,4,5];
    let mut ind = 0;

    while ind != 5 {
        println!("Element at index {ind} is {}", arr[ind]);
        ind += 1;
    }

    // for loops
    let a = [10,20,30,40,50];

    for element in a{
        println!("{element} ")
    }
}