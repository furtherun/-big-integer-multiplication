extern crate rand;
use::rand::{distributions::Uniform, Rng};

use std::fmt::Write;

//const MAX_VEC_SIZE: usize = 1_000_000_000;

const MAX_VEC_SIZE: usize = 1_000_000; //test size

fn new_big_integer(len: usize) -> Option<Vec<u8>> {
    // Retruns a random big integer in vec,
    // or None if out of memory.
    // vec![5, 4, 3, 2, 1] represents the number 12345.

    if len > MAX_VEC_SIZE {
        None
    } else {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, 10);

        let mut big_num: Vec<u8> = (0..len).map(|_| rng.sample(&range))
                                           .collect();

        big_num[0] = rng.gen_range(1..10); 
        Some(big_num)
    }
}
fn get_big_integer_string(num: &Vec<u8>) -> String { 
    // Returns the big integer in string format,
    // vec![5, 4, 3, 2, 1] is converted into "12345".

    let mut big_num_str = String::with_capacity(num.len());
    for n in num.iter().rev() {
        write!(big_num_str, "{}", n).unwrap();
    }
    big_num_str
}

fn muti_big_integer(num1: &Vec<u8>, num2: &Vec<u8>) -> Option<Vec<u8>> {
    // We assume that num1 and num2 are of the same length,
    // both of them have no leading zeros.
    
    let mut big_num: Vec<u8> = vec![0; num1.len()+num2.len()];

    for i in 0..num1.len() {
        for j in 0..num2.len() {
            let mul = num1[i] * num2[j];

            let curr: usize = i + j;
            let high: usize = i + j + 1;
            let total: u8 = mul + big_num[curr];

            big_num[high] += total / 10;
            big_num[curr] = total % 10;
        }
    }

    //remove leading 0's
    while !big_num.is_empty() && big_num[big_num.len()-1] == 0 {
        big_num.pop().unwrap();
    }

    Some(big_num)
}

fn run_muti_case() {
    let mut len = 10;
                                                                               
    loop {
        match (new_big_integer(len), new_big_integer(len)) {
            (Some(num1), Some(num2)) => {
                let string1 = get_big_integer_string(&num1);
                let string2 = get_big_integer_string(&num2);
                                                                               
                println!("The origin number is {} and {}.", string1, string2);
                                                                               
                match muti_big_integer(&num1, &num2) {
                    Some(num) => {
                        let bn_str = get_big_integer_string(&num);
                                                                               
                        println!("The mutiply result is {}.", bn_str);
                    }
                    _ => (),
                }
                                                                               
            }
            _ => break,
        };
                                                                               
                                                                               
        len *= 10;
        if len > 1000 {
            break;
        }
                                                                               
    };
}
fn main() {

    run_muti_case();
    
}
