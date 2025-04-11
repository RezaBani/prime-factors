pub fn factors(n: u64) -> Vec<u64> {
    let mut output = Vec::with_capacity(1);

    let mut n = n;
    while n != 1 {
        let i = (2..=n).filter(|i| n % i == 0).next().unwrap();
        n /= i;
        output.push(i);
        continue;
    }
    output
}
