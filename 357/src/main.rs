#![feature(step_by)]

extern crate time;
extern crate bit_vec;


use time::precise_time_s;
use std::collections::BTreeSet;
use bit_vec::BitVec;

fn get_primes_below(max_digit: usize)-> BTreeSet<usize> {
    let mut sieve = BitVec::from_elem(max_digit, true);
    let stop = sieve.len();
    let top = (max_digit as f64).sqrt() as usize + 1;
    for i in (3..top).step_by(2) {
        if sieve[i] == true {
            for j in (i*i..stop).step_by(2*i) {
                sieve.set(j, false);
            }
        }
    }
    let mut result = Box::new(BTreeSet::new());
    result.insert(2);
    for i in (3..max_digit).step_by(2) {
        if sieve[i] == true {
            result.insert(i);
        };
    }
    *result
}

fn check(n: usize, primes: &BTreeSet<usize>) -> bool {
    if ![0usize, 2, 8].contains(&(n % 10))
    {
        return false;
    }
    if !primes.contains(&(n / 2 + 2))
    {
        return false;
    }
    let top = (n as f64).sqrt() as usize;
    for d in 3..top {
        if (n % d) == 0
        {
            if !primes.contains(&(d + n / d)) {
                return false;
            }
        }
    }
    return true;
}

fn find(n: usize) -> usize {
    let start_time = precise_time_s();
    let primes = Box::new(get_primes_below(n));
    let mut sum = 7usize;
    for x in primes.iter() {
        let _x = x - 1;
        if check(_x, &primes) == true {
            sum += _x;
        }
    }

    println!("{} sec.", precise_time_s() - start_time);
    sum
}

fn main() {
    let n = 1000_000_00;
    let result = find(n);
    println!("answer = {}", result);
}


#[test]
fn it_works() {
    let n = 1000_000_00usize;
    assert!(find(n) == 1739023853137);
}
