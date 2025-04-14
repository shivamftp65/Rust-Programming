
struct User{
    is_active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main(){
    let user1:User = User{
        is_active: true,
        username: String::from("shivam@123"),
        email: String::from("shivam@gmail.com"),
        sign_in_count: 1
    };
    // to perform modification in the fields of Structure, we have to make the instance mutable.
    // Eg.
    let mut user2:User = User{
        is_active: true,
        username: String::from("shivam@65"),
        email: String::from("shivam123@gmail.com"),
        sign_in_count: 1
    };

    println!("First User: {}, Second User: {}", user1.username, user2.username);

    // user1.is_active = false; // cannot assign to `user1.is_active`, as `user1` is not declared as mutable
    user2.is_active = false;
    println!("{}", user2.is_active);

}