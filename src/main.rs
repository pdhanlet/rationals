use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Rem, Neg, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use std::cmp::Ordering;

extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};


fn calc_next_prime(primes: &mut Vec<u32>, potential_prime: &mut u32) -> u32 {
    loop {
        let mut is_prime = true;
        for prime in &*primes {
            if *potential_prime % *prime == 0 {
                is_prime = false;
                break;
            }
            if *prime as f32 > *potential_prime as f32 / 2.0 {
                break;
            }
        }
        if is_prime {
            primes.push(*potential_prime);
            *potential_prime += 2;
            break;
        }
        *potential_prime += 2;
    }
    *primes.last().unwrap()
}

fn get_prime_factors(mut x: u32) -> Vec<u32> {
    let prime_data = YamlLoader::load_from_str(include_str!("../data/prime_numbers.yml")).unwrap();
    let prime_data = &prime_data[0];

    let mut potential_prime = prime_data["potential_prime"].as_i64().unwrap() as u32;
    let mut primes: Vec<u32> = Vec::new();
    for prime in prime_data["primes"].as_vec().unwrap() {
        primes.push(prime.as_i64().unwrap() as u32);
    }

    let mut prime_factors: Vec<u32> = Vec::new();
    let mut prime_i = 0;
    while x != 1 {
        let potential_pf = primes[prime_i];
        loop {
            if x % potential_pf == 0 {
                x /= potential_pf;
                prime_factors.push(potential_pf);
                continue;
            }
            break;
        }
        prime_i += 1;
        if prime_i == primes.len() {
            calc_next_prime(&mut primes, &mut potential_prime);
        }
    }
    // Write to yaml here
    prime_factors
}

fn get_hcf(x: u32, y: u32) -> u32 {
    let mut hcf = 1;
    let mut i = 0;
    let mut x_pf = get_prime_factors(x);
    let mut y_pf = get_prime_factors(y);
    loop {     
        if i == x_pf.len() {
            break;
        }
        match y_pf.iter().position(|&n| n == x_pf[i]) {
            Some(j) => {
                hcf *= x_pf[i];
                x_pf.remove(i);
                y_pf.remove(j);
            }
            None => {
                i += 1;
            } 
        }
    }
    hcf
} 

fn get_lcm(x: u32, y:u32) -> u32 {
    let mut lcm = 1;
    let x_pf = get_prime_factors(x);
    let mut y_pf = get_prime_factors(y);
    for pf in x_pf {
        lcm *= pf;
        match y_pf.iter().position(|&n| n == pf) {
            Some(i) => {
                y_pf.remove(i);
            }
            None => ()
        }
    }
    for pf in y_pf {
        lcm *= pf;
    }
    lcm
}

#[derive(Copy, Clone)]
enum Sign {
    Positive,
    Negative,
    Signless
}

impl Sign {
    fn signof(x: i32) -> Sign {
        match x.cmp(&0) {
            Ordering::Greater => Sign::Positive,
            Ordering::Less => Sign::Negative,
            Ordering::Equal => Sign::Signless
        }
    }

    fn as_i32(&self) -> i32 {
        match self {
            Sign::Positive => 1,
            Sign::Negative => -1,
            Sign::Signless => 0
        }
    }
}

impl Mul for Sign {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self == Sign::Signless || other == Sign::Signless {
            return Sign::Signless;
        }
        if self == other {
            return Sign::Positive
        }
        Sign::Negative
    }
}

impl Neg for Sign {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Sign::Negative => Sign::Positive,
            Sign::Signless => Sign::Signless,
            Sign::Positive => Sign::Negative,
        }
    }
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sign::Negative => f.write_str("-"),
            _ => f.write_str("")
        }
    }
}

impl fmt::Debug for Sign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sign::Positive => f.write_str("Positive"),
            Sign::Negative => f.write_str("Negative"),
            Sign::Signless => f.write_str("Signless")
        }
    }
}

#[derive(Copy, Clone)]
struct Rational {
    num: u32,
    denom: u32,
    sign: Sign,
}

impl Rational {
    fn new(num: i32, denom: i32) -> Rational {
        if denom == 0 {
            panic!("Fraction denominator cannot be 0");
        }
        let mut fraction = Rational {
            num: num.abs() as u32,
            denom: denom.abs() as u32,
            sign: Sign::signof(num) * Sign::signof(denom)
        };
        fraction.simplify();
        fraction
    }

    // fn from_f64(x: f64) -> Fraction {

    // }

    fn as_f64(&self) -> f64 {
        self.num as f64 / self.denom as f64 * self.signum() as f64
    }

    fn simplify(&mut self) {
        if self.num == 0 {
            self.denom = 1;
            self.sign = Sign::Signless;
            return ();
        }
        let hcf = get_hcf(self.num, self.denom);
        self.num /= hcf;
        self.denom /= hcf;
    }

    fn signum(&self) -> i32 {
        self.sign.as_i32()
    }

    fn truc(&mut self) {

    }

    
}

impl Add for Rational {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let lcm = get_lcm(self.denom, other.denom);
        let num = (self.num * lcm / self.denom) as i32 * self.sign.as_i32()
                     + (other.num * lcm / other.denom) as i32 * other.sign.as_i32();
        Rational { 
            num: num.abs() as u32, 
            denom: lcm, 
            sign: Sign::signof(num)
        }
    }
}

impl Sub for Rational {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let lcm = get_lcm(self.denom, other.denom);
        let num = (self.num * lcm / self.denom) as i32 * self.sign.as_i32()
                     - (other.num * lcm / other.denom) as i32 * other.sign.as_i32();
        Rational { 
            num: num.abs() as u32, 
            denom: lcm, 
            sign: Sign::signof(num)
        }
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let frac = Rational {
            num: self.num * other.num,
            denom: self.denom * other.denom,
            sign: self.sign * other.sign
        };
        frac.simplify();
        frac
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let frac = Rational {
            num: self.num * other.denom,
            denom: self.denom * other.num,
            sign: self.sign * other.sign
        };
        frac.simplify();
        frac
    }
}

impl Rem for Rational {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        let q = (self / other).as_f64().floor();
        self - other * q
    }
}

impl Neg for Rational {
    type Output = Self;

    fn neg(self) -> Self {
        Rational {
            sign: -self.sign,
            ..self
        }
    }
}

impl AddAssign for Rational {
    fn add_assign(&mut self, other: Self) {
        self = self + other;
    }
}

impl SubAssign for Rational {
    fn add_assign(&mut self, other: Self) {
        self = self - other;
    }
}

impl MulAssign for Rational {
    fn add_assign(&mut self, other: Self) {
        self = self * other;
    }
}

impl DivAssign for Rational {
    fn add_assign(&mut self, other: Self) {
        self = self / other;
    }
}

impl RemAssign for Rational {
    fn add_assign(&mut self, other: Self) {
        self = self % other;
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}{}/{}", self.sign, self.num, self.denom)
    }
}

impl fmt::Debug for Rational {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Fraction[num: {}, denom: {}, sign: {:?}]", self.num, self.denom, self.sign)
    }
}

fn main() {
    // dbg!(get_prime_factors(2));
    // dbg!(get_prime_factors(1));
    // // dbg!(get_prime_factors(0));
    // dbg!(get_prime_factors(10));
    // dbg!(get_prime_factors(11));
    // dbg!(get_prime_factors(12));
    // dbg!(get_prime_factors(13));

    // dbg!(get_lcm(1, 1));
    // dbg!(get_lcm(1, 2));
    // dbg!(get_lcm(2, 2));
    // dbg!(get_lcm(4, 6));
    // dbg!(get_lcm(3, 9));
    // dbg!(get_lcm(128, 24));
    // dbg!(get_lcm(24, 128));
    // dbg!(get_lcm(56, 132));
    // dbg!(get_lcm(99, 367));
    // dbg!(get_lcm(201, 3859));
    // dbg!(get_lcm(423, 57));
    
    // println!("{}", Fraction::new(-2, 4));
    // println!("{}", Fraction::new(42, 322));
    // println!("{}", Fraction::new(57, 1273));
    // println!("{}", Fraction::new(100, -5));
    // println!("{}", Fraction::new(-42, -366));
    // println!("{}", Fraction::new(69, 69));
    // println!("{}", Fraction::new(0, 1));
    // println!("{}", Fraction::new(-0, 5));
    // println!("{}", Fraction::new(0, -10));
    // println!("{}", Fraction::new(3, 0));
    // println!("{}", Fraction::new(0, 0));

    // let frac = Fraction::new(1, 2);
    // let other = frac;
    // println!("{}, {}", frac, other); 
    // dbg!(Fraction::new(1, 1) + Fraction::new(1, 1));
    println!("{}", Rational::new(1, 1) + Rational::new(1, 1));
    println!("{}", Rational::new(5, 6) + Rational::new(4, 3));
    println!("{}", Rational::new(1, -3) + Rational::new(-1, -2));
    println!("{}", Rational::new(46, 54) + Rational::new(-7, 9));
    println!("{}", Rational::new(2, -11) + Rational::new(3, -12));
    println!("{}", Rational::new(4, 73) + Rational::new(208, 1999));
    // println!("{}", Fraction::new(1, 1) + Fraction::new(1, 1));
    // println!("{}", Fraction::new(1, 1) + Fraction::new(1, 1));
    // println!("{}", Fraction::new(1, 1) + Fraction::new(1, 1));
    // println!("{}", Fraction::new(1, 1) + Fraction::new(1, 1));
    // println!("{}", Fraction::new(1, 1) + Fraction::new(1, 1));
    
}
