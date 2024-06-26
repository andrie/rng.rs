// This file is not used, but implements a small part of MT in Rust


const N: usize = 624;
const M: usize = 397;
const MATRIX_A: i32 = 0x9908b0dfu32 as i32;
const UPPER_MASK: i32 = 0x80000000u32 as i32;
const LOWER_MASK: i32 = 0x7fffffff;

static mut MT: [i32; N] = [0; N];
static mut MTI: usize = N + 1;

pub fn seed_mt(seed: i32) {
    unsafe {
        MT[0] = seed;
        for i in 1..N {
            MT[i] = 1812433253 * (MT[i - 1] ^ (MT[i - 1] >> 30)) + i as i32;
        }
        MTI = N;
    }
}

pub fn extract_number() -> f64 {
    if unsafe { MTI >= N } {
        if unsafe { MTI == N + 1 } {
            seed_mt(5489);
        }
        twist();
    }

    let mut y: i32;
    unsafe {
        y = MT[MTI];
        MTI += 1;
    }

    y ^= y >> 11;
    y ^= (y << 7) & 0x9d2c5680u32 as i32;
    y ^= (y << 15) & 0xefc60000u32 as i32;
    y ^= y >> 18;

    y as f64 / std::i32::MAX as f64
}

fn twist() {
    for i in 0..N {
        let x = (unsafe { MT[i] } & UPPER_MASK) + (unsafe { MT[(i + 1) % N] } & LOWER_MASK);
        let x_a = x >> 1;
        unsafe {
            MT[i] = MT[(i + M) % N] ^ x_a;
            if x % 2 != 0 {
                MT[i] ^= MATRIX_A;
            }
        }
    }
    unsafe {
        MTI = 0;
    }
}