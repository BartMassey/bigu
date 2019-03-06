extern crate num;

use num::BigUint;

fn main() {
    let mut b: BigUint = 1u32.into();

    for i in 0u32..100_000 {
        b += &b * i;
    }

    println!("{}", b);
}
