

use rnorm::{r_set_seed, runif, rnorm, epoch_seed};


fn main() {

    r_set_seed(epoch_seed());
    r_set_seed(42);

    println!("runif: {:.5?}", runif(Some(10)));
    
    r_set_seed(42);
    println!("rnorm: {:.5?}", rnorm(Some(5), 0.0, 1.0));
}
