extern crate num;

use num::BigUint;

fn main() {
    let mut b: BigUint = 1u32.into();

    for i in 1..=100_000u32 {
        b += &b * i;
    }

    println!("{}", b);
}
