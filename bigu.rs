fn main() {
    let mut b: num::BigUint = 1u32.into();

    for i in 0u32..100_000 {
        b += &b * i;
    }

    println!("{}", b);
}
