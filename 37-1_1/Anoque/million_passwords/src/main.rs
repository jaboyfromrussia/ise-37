extern crate rand;

use rand::Rng;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n: i32 = s.trim().parse().expect("Is not number");
    
    let lol:Vec<char> = "qwertyuiopasdfghjklzxcvbnm1234567890".chars().collect();
    let mut password = String::new();

    for _i in 0..1000000 {
        for _k in 0..n {
            password.push(lol[rand::thread_rng().gen_range(0, lol.len())]);
        }

        println!("{}", password);
        password = String::new();
    }
}
