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

        big_num[len-1] = rng.gen_range(1..10); 
        Some(big_num)
    }
}
fn get_big_integer_string(num: &[u8]) -> String { 
    // Returns the big integer in string format,
    // vec![5, 4, 3, 2, 1] is converted into "12345".

    let mut big_num_str = String::with_capacity(num.len());
    for n in num.iter().rev() {
        write!(big_num_str, "{}", n).unwrap();
    }
    big_num_str
}

fn multi_big_integer(num1: &[u8], num2: &[u8]) -> Option<Vec<u8>> {
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

    //Remove leading 0's
    while !big_num.is_empty() && big_num[big_num.len()-1] == 0 {
        big_num.pop().unwrap();
    }

    Some(big_num)
}
#[test]
fn test_multi_big_integer() {
    assert_eq!(multi_big_integer(&[4,2,2,1,3,7,9,6,5,5],
                                 &[3,0,2,2,1,6,3,2,0,6],),
               Some(vec![2,7,4,6,2,5,6,1,3,8,6,9,0,0,9,9,4,5,3,3]));
}
fn run_multi_case() {
    let mut len = 10;
                                                                               
    loop {
        match (new_big_integer(len), new_big_integer(len)) {
            (Some(num1), Some(num2)) => {
                let string1 = get_big_integer_string(&num1);
                let string2 = get_big_integer_string(&num2);
                                                                               
                println!("The origin number is {} and {}.", string1, string2);
                                                                               
                match multi_big_integer(&num1, &num2) {
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
fn add_big_integer(num1: &[u8], num2: &[u8]) -> Option<Vec<u8>> {
    //We assume that the length of num1 is not less than that of num2.
    //Example:
    //num1: [4, 2, 0, 1] represents "1024", num2: [9, 8, 7] represents "789".

    if num2.len() > num1.len() {
        return add_big_integer(num2, num1);
    }

    let mut big_num = num1.to_vec();
    big_num.push(0); //add 0 at highest postion

    let mut carry = 0;

    for i in 0..big_num.len() {
        if i < num2.len() {
            big_num[i] += num2[i] + carry;
        } else {
            big_num[i] += carry;
        }
        carry = big_num[i] / 10;
        big_num[i] = big_num[i] % 10;
            
    }
    //Remove leading 0's
    while !big_num.is_empty() && big_num[big_num.len()-1] == 0 {
        big_num.pop().unwrap();
    }

    Some(big_num)
}
#[test]
fn test_add_big_integer() {
    assert_eq!(add_big_integer(&[4, 2, 0, 1], &[9, 8, 7]),
               Some(vec![3, 1, 8, 1]));
    assert_eq!(add_big_integer(&[9, 8, 7], &[4, 2, 0, 1]),
              Some(vec![3, 1, 8, 1]));
}

fn multi_big_integer_recursion(num1: &[u8], num2: &[u8]) -> Option<Vec<u8>> {
    // We assume that num1 and num2 are of the same length,
    // both of them have no leading zeros.
    // num1 = a1 * 10^{n/2} + a0, num2 = b1 * 10^{n/2} + b0
    // num1 * num2 = a0b0 + (a0b1+a1b0) * 10^{n/2} + a1b1 * 10^{n}

    if num1.len() == 1 {
        return multi_big_integer(&num1, &num2);
    } 
    
    let a0 = &num1[..num1.len()/2]; //low part of num1
    let a1 = &num1[num1.len()/2..]; //high part of num1
    let b0 = &num2[..num2.len()/2]; //low part of num2
    let b1 = &num2[num2.len()/2..]; //high part of num2

    match (multi_big_integer_recursion(a0, b0),
           multi_big_integer_recursion(a1, b1),
           multi_big_integer_recursion(a0, b1),
           multi_big_integer_recursion(a1, b0)) {
        (Some(a0b0), Some(a1b1), Some(a0b1), Some(a1b0)) => {
            match add_big_integer(&a0b1, &a1b0) {
                Some(add_sum) => {

                    let mut mid = add_sum.clone();
                    mid.reverse();
                    mid.append(&mut vec![0; num1.len()/2]);
                    mid.reverse();

                    let mut high = a1b1.clone();
                    high.reverse();
                    high.append(&mut vec![0; num1.len()]);
                    high.reverse();
                    match add_big_integer(&mid, &high) {
                        Some(tmp) => add_big_integer(&tmp, &a0b0),
                        _ => None,
                    }
                }            
                _ => None,
            }
        }
        _ => None,
    }
}
#[test]
fn test_multi_big_integer_recursion() {
    assert_eq!(multi_big_integer_recursion(&[1], &[1]), Some(vec![1]));
    
    assert_eq!(multi_big_integer_recursion(&[2,1], &[8,1]), Some(vec![6,1,2]));
    assert_eq!(multi_big_integer_recursion(&[4,2,0,1], &[9,8,7,1]),
              Some(vec![6,3,9,1,3,8,1]));

    //assert_eq!(multi_big_integer_recursion(&[4,2,2,1,3,7,9,6,5,5],
    //                                       &[3,0,2,2,1,6,3,2,0,6],),
    //       Some(vec![2,7,4,6,2,5,6,1,3,8,6,9,0,0,9,9,4,5,3,3]));
}

fn _multi_big_integer_recursion_plus(num1: &[u8], num2: &[u8]) -> Option<Vec<u8>> {
    // We assume that num1 and num2 are of the same length,
    // both of them have no leading zeros.
    // num1 = a1 * 10^{n/2} + a0, num2 = b1 * 10^{n/2} + b0
    // num1 * num2 = a0b0 + (a0b1+a1b0) * 10^{n/2} + a1b1 * 10^{n}
    //             = a0b0 + ((a0-a1)(b1-b0) + a0b0 + a1b1) * 10^{n/2} 
    //                    + a1b1 * 10^{n}
    //
    // Take care! (a0-a1)(b1-b0) must be a negtive number! Prove it. 

    if num1.len() == 1 {
        return multi_big_integer(&num1, &num2);
    } 
    
    let a0 = &num1[..num1.len()/2]; //low part of num1
    let a1 = &num1[num1.len()/2..]; //high part of num1
    let b0 = &num2[..num2.len()/2]; //low part of num2
    let b1 = &num2[num2.len()/2..]; //high part of num2

    match (multi_big_integer_recursion(a0, b0),
           multi_big_integer_recursion(a1, b1),
           multi_big_integer_recursion(a0, b1),
           multi_big_integer_recursion(a1, b0)) {
        (Some(a0b0), Some(a1b1), Some(a0b1), Some(a1b0)) => {
            match add_big_integer(&a0b1, &a1b0) {
                Some(add_sum) => {

                    let mut mid = add_sum.clone();
                    mid.reverse();
                    mid.append(&mut vec![0; num1.len()/2]);
                    mid.reverse();

                    let mut high = a1b1.clone();
                    high.reverse();
                    high.append(&mut vec![0; num1.len()]);
                    high.reverse();
                    match add_big_integer(&mid, &high) {
                        Some(tmp) => add_big_integer(&tmp, &a0b0),
                        _ => None,
                    }
                }            
                _ => None,
            }
        }
        _ => None,
    }
}

fn main() {

    multi_big_integer_recursion(&[2,1], &[8,1]);
    run_multi_case();
    
}
