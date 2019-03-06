// https://www.reddit.com/r/rust/comments/axxy73/very_new_to_rust_wanted_to_test_it_got_bitten/ehwtj1l/

use rug::{Assign, Integer};

fn main() {
    let mut b = Integer::new();
    b.assign(1);

    for i in 0..100_000 {
        b += b.clone() * i;
    }

    println!("{}", b);
}
