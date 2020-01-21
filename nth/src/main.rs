use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: usize = args[1].parse().unwrap();
    println!("{:?}", nth_prime(n));
}

pub fn nth_prime(n: usize) -> usize {
    let mut primes: Vec<usize> = Vec::with_capacity(n);

    // Find upper limit of number range.
    let mut nums: Vec<usize> = if n >= 6 {
        // pn < n ln (n ln n) for n ≥ 6
        let n_fl = n as f64;
        let max = ((n_fl * ((n_fl * n_fl.ln()).ln())).ceil() as usize).into();
        (2..=max).collect()
    } else {
        (2..=13).collect()
    };

    while !nums.is_empty() {
        // Add next prime
        let p = nums[0];
        primes.push(p);
        // Remove next prime multiples from list.
        nums = nums.into_iter().filter(|x| x % p != 0).collect();
    }
    primes[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth() {
        assert!(nth_prime(1) == 2);
        assert!(nth_prime(2) == 3);
        assert!(nth_prime(3) == 5);
        assert!(nth_prime(4) == 7);
        assert!(nth_prime(5) == 11);
        assert!(nth_prime(6) == 13);
        assert!(nth_prime(100) == 541);
        assert!(nth_prime(1000) == 7919);
        assert!(nth_prime(5000) == 48_611);
    }
}
