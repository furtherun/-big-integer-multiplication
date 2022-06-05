mod big_integer;
use big_integer::*;

//mod run;
//use run::*;

type BI = BigInteger;

fn main() {

    //let mut bi1 = BI::from("110");
    //let mut bi2 = BI::from("990");
    //let res = bi1.mult_recur(&mut bi2);
    //let res = bi1.mult_recur_pro(&mut bi2);
    //println!("{}", res);

    //let mut len = 10;
    //let mut bi_x = BI::rand_init(len);
    //let mut bi_y = BI::rand_init(len);
    //println!("x = {bi_x}, y = {bi_y}");
    //let res1 = bi_x.mult(&bi_y);
    //let res2 = bi_x.mult(&mut bi_y);
    //let res3 = bi_x.mult_recur_pro(&mut bi_y);
    //println!("{bi_x}*{bi_y}");
    //println!("res1={res1}");
    //println!("res2={res2}");
    //println!("res3={res3}");

    //run(true);
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
#[test]
fn test_add() {
    assert_eq!(BI::from("123").add(&BI::from("456")),
               BI::from("579"));
    assert_eq!(BI::from("0").add(&BI::from("9870")),
               BI::from("9870"));
    assert_eq!(BI::from("999").add(&BI::from("11")),
               BI::from("1010"));
}
#[test]
fn test_mult_recur() {
    assert_eq!(BI::from("1234").mult_recur(&mut BI::from("7890")),
               BI::from("9736260"));
    assert_eq!(BI::from("0").mult_recur(&mut BI::from("9")),
               BI::from("0"));
    assert_eq!(BI::from("1234").mult_recur(&mut BI::from("2")),
               BI::from("2468"));
}
#[test]
fn test_add_hl() {
    assert_eq!(BI::from("123").add_hl(),
               BI::from("15"));
    assert_eq!(BI::from("7899").add_hl(),
               BI::from("177"));
    assert_eq!(BI::from("456").add_hl(),
               BI::from("51"));
}
#[test]
fn test_sub() {
    assert_eq!(BI::from("7890").sub(&BI::from("1234")),
               BI::from("6656"));
}
#[test]
fn test_mult_recur_pro() {
    assert_eq!(BI::from("1234").mult_recur_pro(&mut BI::from("7890")),
               BI::from("9736260"));
    assert_eq!(BI::from("0").mult_recur_pro(&mut BI::from("9")),
               BI::from("0"));
    assert_eq!(BI::from("999").mult_recur_pro(&mut BI::from("280")),
               BI::from("279720"));
}
