const CAPACITY: usize = 1_000_000_000; 

#[derive(PartialEq, Debug)]
pub struct BigInteger {

    pub start: usize,
    pub len: usize,
    pub nums: Vec::<i32>,
}

impl BigInteger {
    pub fn new() -> BigInteger {
        BigInteger {
            start: 0,
            len: 0,
            nums: vec![0; CAPACITY]
        }
    }
    //pub fn init(len: usize) -> BigInteger {
    //    
    //}
    pub fn from(s: &str) -> BigInteger {
        let mut bi = BigInteger::new();
        bi.len =  s.len();

        for (i, c) in s.chars().rev().enumerate() {
            bi.nums[i] = c.to_digit(10).unwrap() as i32;
        }

        bi
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn to_string(&self) -> String {
        self.nums.get(self.start..self.start+self.len).unwrap()
            .iter().rev()
            .map(|i| i.to_string()).collect::<String>()
    }
    pub fn mult (&self, other: &BigInteger) -> BigInteger {
        let mut bi = BigInteger::new();
        
        bi.start = self.start + other.start;
        bi.len = self.len + other.len;
        
        for i in self.start..self.start+self.len {
            for j in other.start..other.start+other.len {
                let mul = self.nums[i] * other.nums[j];

                let curr = i + j;
                let high = i + j + 1;
                let total = mul + bi.nums[curr];

                bi.nums[high] += total / 10;
                bi.nums[curr] = total % 10;
            }
        }

        while bi.len > 1 && bi.nums[bi.start+bi.len-1] == 0 {
            bi.len -= 1;
        }

        bi
    }
}

use std::fmt;

impl fmt::Display for BigInteger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


