extern crate rand;
use rand::*;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    let (how_many, size_of_str) = validate_args(args);
    generate_all(how_many, size_of_str);
}

fn validate_args(args: Vec<String>) -> (i32, i32) {
    let len: i32 = args.len() as i32;
    if len == 1 {
        println!("\nDefaulting to 6 characters length and 6 total words\n---------------------------------------------------");
        return (6, 6);
    }
    for arg in &args {
        if arg == "help" {
            println!("\nUsage:\ncargo run <how-many-words> <number-of-characters>");
            println!("\n<how-many-words>       : integer");
            println!(
                "<number-of-characters> : integer (Note: <number-of-characters> must be even)\n"
            );
            process::exit(0x0000);
        }
    }
    let how_many: i32;
    let mut size_of_str: i32;
    if args[1].parse::<i32>().is_ok() {
        how_many = args[1].parse::<i32>().unwrap();
    } else {
        println!(
            "\nCannot parse i32: `{}`. <how-many-words> must be an integer",
            args[1]
        );
        process::exit(0x0001);
    }
    if args[2].parse::<i32>().is_ok() {
        size_of_str = args[2].parse::<i32>().unwrap();
    } else {
        println!(
            "\nCannot parse i32: `{}` . <number-of-characters> must be an integer",
            args[2]
        );
        process::exit(0x0001);
    }

    if size_of_str < 6 {
        println!("Minimum length of word is 6 ..Defaulting to 6\n----------------------------------------------");
        size_of_str = 6;
    }

    return (how_many, size_of_str);
}

fn generate(n: i32) -> String {
    let vowels = vec!['A', 'E', 'I', 'O', 'U'];
    let consonants = vec![
        'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W',
        'X', 'Y', 'Z',
    ];
    let len_vow: i32 = vowels.len() as i32;
    let len_con: i32 = consonants.len() as i32;
    let mut res = String::new();
    for _ in 0..n / 2 {
        let mut rng = rand::thread_rng();
        let index_vow = rng.gen_range(0, len_vow) as usize;
        let index_con = rng.gen_range(0, len_con) as usize;
        let c1 = consonants[index_con];
        let c2 = vowels[index_vow];
        res.push(c1);
        res.push(c2);
    }
    return res;
}

fn generate_all(how_many: i32, size_of_str: i32) {
    assert!(
        size_of_str % 2 == 0,
        "Size of Word must be even; See `cargo run help`"
    );
    for _ in 0..how_many {
        println!("{}", generate(size_of_str));
    }
}
