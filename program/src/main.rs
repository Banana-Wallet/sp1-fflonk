//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

mod fflonk_utils;
mod fflonk;
pub use crate::fflonk::run_main_fflonk;
mod zksync;
mod zksync_utils;
pub use crate::zksync::verify;

pub fn main() {
    // NOTE: values of n larger than 186 will overflow the u128 type,
    // resulting in output that doesn't match fibonacci sequence.
    // However, the resulting proof will still be valid!
    let n = sp1_zkvm::io::read::<u32>();
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut sum: u32;
    for _ in 1..n {
        sum = a + b;
        a = b;
        b = sum;
    }

    // run_main_fflonk();
    verify();


   
    sp1_zkvm::io::commit(&n);
    sp1_zkvm::io::commit(&a);
    sp1_zkvm::io::commit(&b);
}


