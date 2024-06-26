// rng_init.rs

// This function implements RNG_init from the original C code in base R, but only for Mersenne Twister
// https://github.com/wch/r-source/blob/trunk/src/main/RNG.c#L187
// Thanks to https://stats.stackexchange.com/questions/592922/what-is-b-d-ripleys-method-of-seeding-the-mersenne-twister-rng
pub fn rng_init_MT(seed: i32) -> Vec<i32> {
    /* Initial scrambling */

    let mut seed = seed;
    for _ in 0..51 {
        seed = seed.wrapping_mul(69069).wrapping_add(1);
    }   

    let mut vec = Vec::new();
    for _ in 0..624 {
        seed = seed.wrapping_mul(69069).wrapping_add(1);
        vec.push(seed);
    }
    vec
}

// Constants
const N: i32 = 624;
const M: i32 = 397;
const MATRIX_A: i32 = 0x9908b0df;
const UPPER_MASK: i32 = 0x80000000;
const LOWER_MASK: i32 = 0x7fffffff;

// Tempering parameters
const TEMPERING_MASK_B: i32 = 0x9d2c5680;
const TEMPERING_MASK_C: i32 = 0xefc60000;

// Macros translated to inline functions
#[inline]
fn tempering_shift_u(y: i32) -> i32 {
    y >> 11
}

#[inline]
fn tempering_shift_s(y: i32) -> i32 {
    y << 7
}

#[inline]
fn tempering_shift_t(y: i32) -> i32 {
    y << 15
}

#[inline]
fn tempering_shift_l(y: i32) -> i32 {
    y >> 18
}

// Static variables
// allow for optimizing compilers to read over bound
static mut DUMMY: [i32; 628] = [0; 628]; 

// the array for the state vector
// static mut MT: *mut i32 = unsafe { DUMMY.as_mut_ptr().offset(1) }; 

// MTI==N+1 means MT[N] is not initialized
// static mut MTI: i32 = N  + 1 as i32; 

// the array for the state vector
static mut MT: Vec<i32> = vec![0; 628]; 

// MTI==N+1 means MT[N] is not initialized
static mut MTI: usize = N + 1; 


// Assuming N, M, MT, MTI, MATRIX_A, UPPER_MASK, and LOWER_MASK are defined in the same scope as this function
pub fn MT_genrand() -> f64 {
    let mut y: i32;
    let mag01 = [0x0, MATRIX_A]; // mag01[x] = x * MATRIX_A for x=0,1

    if MTI >= N { // generate N words at one time
        let mut kk: usize;

        if MTI == N+1 { // if sgenrand() has not been called,
            MT_sgenrand(4357); // a default initial seed is used
        }

        kk = 0;
        while kk < N - M {
            y = (MT[kk] & UPPER_MASK) | (MT[kk+1] & LOWER_MASK);
            MT[kk] = MT[kk+M] ^ (y >> 1) ^ mag01[(y & 0x1) as usize];
            kk += 1;
        }
        while kk < N - 1 {
            y = (MT[kk] & UPPER_MASK) | (MT[kk+1] & LOWER_MASK);
            MT[kk] = MT[kk+(M-N)] ^ (y >> 1) ^ mag01[(y & 0x1) as usize];
            kk += 1;
        }
        y = (MT[N-1] & UPPER_MASK) | (MT[0] & LOWER_MASK);
        MT[N-1] = MT[M-1] ^ (y >> 1) ^ mag01[(y & 0x1) as usize];

        MTI = 0;
    }

    y = MT[MTI];
    MTI += 1;

    // The rest of the function is missing in the provided code
    // Assuming it should return a f64 value derived from y
    // Placeholder return statement

    // Assuming TEMPERING_SHIFT_U, TEMPERING_SHIFT_S, TEMPERING_SHIFT_T, TEMPERING_SHIFT_L, TEMPERING_MASK_B, and TEMPERING_MASK_C are defined in the same scope as this function
    y ^= tempering_shift_u(y);
    y ^= tempering_shift_s(y) & TEMPERING_MASK_B;
    y ^= tempering_shift_t(y) & TEMPERING_MASK_C;
    y ^= tempering_shift_l(y);

    let dummy[0] = MTI;
    (y as f64) * 2.3283064365386963e-10 // reals: [0,1)-interval

}