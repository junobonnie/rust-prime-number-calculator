//use std::io;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Read;
use std::fs::File;

fn main() {
    // let mut n = String::new();
    // io::stdin()
    //     .read_line( &mut n)
    //     .expect("Failed to read line");
    // let n: u128 = n.trim().parse().expect("Please type a number!");

    let mut primes = vec![];

    let mut f = File::open("data.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong reading the file");

    for line in contents.lines() {
        primes.push(line.parse::<u128>().unwrap());
    }

    let mut file = OpenOptions::new().append(true).open("data.txt").expect("create failed");
    let mut number: u128 = primes.last().unwrap() + 1;
    if is_prime(number, &primes) {
        primes.push(number);
        println!("{}", number);
        file.write_all(format!("{}", number).as_bytes()).expect("write failed");
    }
    loop{
        number += 1;
        if is_prime(number, &primes) {
            primes.push(number);
            println!("{}", number);
            file.write_all(format!("\n{}", number).as_bytes()).expect("write failed");
        }
    }
}

fn isqrt(n: u128) -> u128 {
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

fn is_prime(number: u128, primes: &Vec<u128>) -> bool {
    for prime in primes {
        if number % prime == 0 {
            return false;
        }
        else if prime > &isqrt(number) {
            return true;
        }
    }
    true
}