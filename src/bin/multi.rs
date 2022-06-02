mod big_integer;
use big_integer::*;
type BI = BigInteger;

fn main() {
    //let bi = BigInteger {
    //    start: 0,
    //    end: 0,
    //    nums: vec![1, 2, 3, 4]
    //};

    let bi1 = BigInteger::from("1234");
    let bi2 = BigInteger::from("23");
    println!("{}", bi1.mult(&bi2));

    println!("This is multi.rs!");

}


#[test]
fn test_mult() {
    assert_eq!(BI::from("1234").mult(&BI::from("7890")),
              BI::from("9736260"));
    assert_eq!(BI::from("0").mult(&BI::from("9")),
              BI::from("0"));
    assert_eq!(BI::from("1234").mult(&BI::from("2")),
              BI::from("2468"));
}
