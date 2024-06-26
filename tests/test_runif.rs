
use rnorm::{r_set_seed, runif, rnorm}; //, epoch_seed};

fn assert_all_equal(v1: Vec<f64>, v2: Vec<f64>) {
    for i in 0..v1.len() {
        assert!((v1[i] - v2[i]).abs() < 1e-7); // Compare with a small tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runif() {
        r_set_seed(1); // Set the seed to 1
        let result = runif(Some(5)); // Generate 5 random numbers
        let expected = vec![0.2655087, 0.3721239, 0.5728534, 0.9082078, 0.2016819];
        assert_all_equal(result, expected);
    }

    #[test]
    fn test_rnorm() {
        r_set_seed(1); // Set the seed to 1
        let result = rnorm(Some(5), 0.0, 1.0); // Generate 5 random numbers
        let expected = vec![-0.6264538, 0.1836433, -0.8356286, 1.5952808, 0.3295078];
        assert_all_equal(result, expected);
    }
}