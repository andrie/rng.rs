extern "C" {
    fn set_seed(seed: i32);
    fn unif_rand_c() -> f64;
    fn rnorm_c(mu: f64, sigma: f64) -> f64;
}

pub fn epoch_seed() -> i32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_secs() as i32
}

pub fn r_set_seed(seed: i32) {
    // Your implementation here
    unsafe {
        set_seed(seed);
    }
}

// Generates a vector of uniformly distributed random numbers.
///
/// This function generates a vector of `n` uniformly distributed random numbers
/// using the `unif_rand` function from the C library. If `n` is not specified,
/// it defaults to 1.
///
/// # Arguments
///
/// * `n` - An optional parameter that specifies the number of random numbers to generate.
///          If `None`, the function generates a single random number.
///
/// # Examples
///
/// Generate 5 random numbers:
///
/// ```
/// let numbers = r_unif_rand(Some(5));
/// assert_eq!(numbers.len(), 5);
/// ```
///
/// Generate a single random number:
///
/// ```
/// let numbers = r_unif_rand(None);
/// assert_eq!(numbers.len(), 1);
/// ```
pub fn runif(n: Option<usize>) -> Vec<f64> {
    let n = n.unwrap_or(1);
    let mut result = Vec::with_capacity(n);
    let mut r: f64;

    for _ in 0..n {
        unsafe {
            r = unif_rand_c();
            result.push(r);
        };
    }
    result
}

pub fn rnorm(n: Option<usize>, mu: f64, sigma: f64) -> Vec<f64> {
    let n = n.unwrap_or(1);
    let mut result = Vec::with_capacity(n);
    let mut r: f64;

    for _ in 0..n {
        unsafe {
            r = rnorm_c(mu, sigma);
            result.push(r);
        };
    }
    result
}
