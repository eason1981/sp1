//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    let n = core::hint::black_box(1_000_000);
    // let n = core::hint::black_box(2_000_000);
    // let n = core::hint::black_box(4_000_000);
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    for _ in 1..n {
        let sum = a + b;
        a = b;
        b = sum;
    }
    if a == 0 {
        panic!();
    }
}
