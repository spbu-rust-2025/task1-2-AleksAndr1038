use std::io;

fn main() {
    let mut sum = 0;

    loop {
        let mut input: String = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            println!("NaN");
            return;
        }

        let input = input.trim();

        match input.parse::<i32>() {
            Ok(num) => {
                if num == -1 {
                    break;
                }

                if num >= 0 {
                    sum += num;
                } else {
                    println!("NaN");
                    return;
                }
            }

            Err(_) => {
                println!("NaN");
                return;
            }
        }
    }

    println!("{}", sum);
}
