use std::io;

fn main() {
    loop {
        println!("Enter the nth number of Fibonacci sequence to see that Fibonacci number");

        let mut f_number = String::new();

        io::stdin()
            .read_line(&mut f_number)
            .expect("Failed to read the line");

        let f_number: u64 = match f_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number more than zero.");
                continue;
            }
        };

        let mut count: u64 = 2;
        let mut first_f_number: u128 = 0;
        let mut second_f_number: u128 = 1;
        let mut current_f_number: u128 = 0;

        if f_number == 0 {
            println!("Please type a number more than zero.");
            continue;
        } else if f_number == 1 {
            println!("1 Fibonacci number is {first_f_number}");
        } else if f_number == 2 {
            println!("2 Fibonacci number is {second_f_number}");
        } else {
            while count < f_number {
                current_f_number = first_f_number + second_f_number;
                first_f_number = second_f_number;
                second_f_number = current_f_number;
                count += 1;
            }
            println!("{f_number} Fibonacci number is {current_f_number}");
        }
        break;
    }
}
