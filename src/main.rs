// fn add(num_one: i32, num_two: i32) -> i32 {
//     num_one + num_two
// }

struct BankAccount {
    ballance: i32,
    verified: bool,
}

// fn print_ballance(account: &BankAccount) {
//     println!("The ballance is: {:?}", account.ballance);
// }

// fn print_verified(account: &BankAccount) {
//     println!("The ballance is: {:?}", account.verified);
// }
fn main() {
    // println!("Hello, world!");
    // let mut my_name = "Rust";
    // my_name = "Python";

    // let mut total: i32 = add(5, 6);
    // let mut discount: bool = false;

    // println!("The value of foo is: {}", total);
    // println!("{} {}", total, true);
    // println!("{0} {0}", total);
    // println!("{:?}", total);

    // if total > 10 {
    //     println!("The total is greater than 10, you are eligible for a discount");
    //     discount = true;
    // } else if total < 10 {
    //     println!("The total is less than 10, , you are eligible for a discount");
    //     discount = true;
    // } else {
    //     println!("The total is equal to 10, , you are not eligible for a discount");
    // }

    // println!("{:?}", total);

    // total = match discount {
    //     true => total + 0,
    //     false => total + 10
    // };

    // match total {
    //     1 => println!("The total is 1"),
    //     2 => println!("The total is 2"),
    //     11 => println!("The total is 11"),
    //     _ => println!("The total is greater than 10"),
    // };

    // let items: [i32; 5] = [1, 2, 3, 4, 5];
    // println!("{:?}", items);

    // let vector_itemes = vec![1, 2, 3, 4, 5];
    // let mut vector_itemes_2 = Vec::new();
    // vector_itemes_2.push(1);
    // vector_itemes_2.push(2);
    // vector_itemes_2.push(3);
    // vector_itemes_2.push(4);
    // vector_itemes_2.push(5);

    // println!("{:?}", vector_itemes);
    // println!("{:?}", vector_itemes_2);

    let my_account = BankAccount {
        ballance: 100,
        verified: false,
    };

    // println!("{:?}", my_account.ballance);
    // println!("{:?}", my_account.verified);

    // print_ballance(&my_account);
    // print_verified(&my_account);

    let verification_status = is_verified(&my_account).expect("error occured!");
    println!("{:?}", verification_status);
}

fn is_verified(account: &BankAccount) -> Result<bool, bool> {
    match account.verified {
        true => Ok(true),
        false => Err(false),
    }
}
