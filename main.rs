use std::io;

fn main() {
    loop {
        let mut choice: String = String::new();
        let mut a_str: String = String::new();
        let mut b_str: String = String::new();
        let mut c_str: String = String::new();

        println!("Выберете вариант:\n 1.Гипотенуза\n 2.Катет");
        match io::stdin().read_line(&mut choice) {
            Ok(_) => {}
            Err(e) => println!("ОШИБКА ВВОДА: {}", e),
        }

        let num: i8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Введите число!");
                continue;
            }
        };

        match num {
            1 => {
                println!("Введите A:");
                match io::stdin().read_line(&mut a_str) {
                    Ok(_) => {}
                    Err(e) => println!("ОШИБКА ВВОДА: {}", e),
                }
                println!("Введите B:");
                match io::stdin().read_line(&mut b_str) {
                    Ok(_) => {}
                    Err(e) => println!("ОШИБКА ВВОДА: {}", e),
                }
                let a: f32 = a_str.trim().parse().unwrap();
                let b: f32 = b_str.trim().parse().unwrap();
                let hypo: f32 = a.powf(2.0) + b.powf(2.0);

                println!(
                    "a = {}\n b = {}\n c = √{}^2 + {}^2\n c = √{} + {}\n c = √{hypo}\n c = {:.2}",
                    a,
                    b,
                    a,
                    b,
                    a.powf(2.0),
                    b.powf(2.0),
                    hypo.sqrt()
                )
            }
            2 => {
                println!("Введите Катет:");
                match io::stdin().read_line(&mut a_str) {
                    Ok(_) => {}
                    Err(e) => println!("ОШИБКА ВВОДА: {}", e),
                }
                println!("Введите Гипотенузу:");
                match io::stdin().read_line(&mut c_str) {
                    Ok(_) => {}
                    Err(e) => println!("ОШИБКА ВВОДА: {}", e),
                }
                let a: f32 = a_str.trim().parse().unwrap();
                let c: f32 = c_str.trim().parse().unwrap();
                let x_cath: f32 = c.powf(2.0) - a.powf(2.0);

                println!(
                    "a = {}\n c = {}\n b = √{}^2 - {}^2\n b = √{} - {}\n c = √{x_cath}\n b = {}",
                    a,
                    c,
                    c,
                    a,
                    c.powf(2.0),
                    a.powf(2.0),
                    x_cath.sqrt()
                )
            }
            _ => println!("Выберете 1 или 2 вариант!!!"),
        }
    }
}
