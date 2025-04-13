// loop, while, for

fn main(){
    // loop

    let mut counter = 1;
    let result = loop {
        counter = counter+1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{result}");

    // loop inside the loop

    let mut iteration = 0;
    'counting_up: loop {
        println!("iteration = {iteration}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if iteration == 2 {
                break 'counting_up;
            }
            remaining  -= 1;
        }

        iteration += 1;
    }

    println!("End iteration = {iteration}");
}
