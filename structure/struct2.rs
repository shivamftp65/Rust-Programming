struct User{
    is_active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main(){
    let user1 = build_user(String::from("shivam123"), String::from("shivam@gmail.com"));
    let mut user2 = build_user(String::from("shivam65"), String::from("shivam1234@gmail.com"));
    println!("{}", user1.username);
    // user1.is_active = false; cannot assign to `user1.is_active`, as `user1` is not declared as mutable
    
    // Now you can update the fields of user2
    // user2.username = String::from("shivamftp65");
    // user2.is_active = false;

    // println!("{}: {}", user2.username, user2.is_active);


    let user3 = User{
        is_active: user1.is_active,
        username: user1.username,
        email: String::from("user3@gmail.com"),
        sign_in_count: user1.sign_in_count
    };
    // We do the same by using this syntax
    let user4 = User{
        email: String::from("user4@gmail.com"),
        ..user2
    };

    println!("{} {}", user3.username, user3.email);
    println!("{} {}", user4.username, user4.email);

}

fn build_user(username: String, email:String) -> User{
    User{
        is_active: true,
        username,
        email,
        sign_in_count: 1
    }
}