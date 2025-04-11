pub fn factors(n: u64) -> Vec<u64> {
    let mut output = Vec::with_capacity(1);
    let primes = calc_primes(n);

    let mut n = n;
    'outer: while n != 1 {
        for prime in &primes {
            if n % prime == 0 {
                n = n / prime;
                output.push(*prime);
                continue 'outer;
            }
        }
    }

    output
}

fn calc_primes(n: u64) -> Vec<u64> {
    let mut num = 2;
    let mut primes = Vec::new();
    while num < n + 1 {
        if primes.iter().any(|prime| num % prime == 0) {
            num += 1;
            continue;
        }
        primes.push(num);
    }
    primes
}
