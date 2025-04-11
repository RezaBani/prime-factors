pub fn factors(n: u64) -> Vec<u64> {
    let mut output = Vec::with_capacity(1);
    let mut primes = Vec::new();

    let mut num = 2;
    let mut n = n;
    while n != 1 {
        let prime = primes.iter().filter(|prime| n % *prime == 0).take(1).next();
        match prime {
            Some(prime) => {
                n /= prime;
                output.push(*prime);
                continue;
            }
            None => {
                update_primes(&mut primes, &mut num);
            }
        }
    }
    output
}

fn update_primes(primes: &mut Vec<u64>, num: &mut u64) {
    loop {
        if !primes.iter().any(|prime| *num % prime == 0) {
            primes.push(*num);
            break;
        }
        *num += 1;
    }
}
