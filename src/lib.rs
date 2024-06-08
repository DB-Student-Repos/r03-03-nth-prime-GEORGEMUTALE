pub fn nth(n: u32) -> u32 {
    // Helper function to check if a number is prime
    fn is_prime(num: u32) -> bool {
        num > 1 && (num == 2 || (num % 2 != 0 && !(3..=((num as f64).sqrt() as u32)).step_by(2).any(|i| num % i == 0)))
    }

    // Finding the nth prime
    (1..).filter(|&x| is_prime(x)).nth(n as usize).unwrap()
}