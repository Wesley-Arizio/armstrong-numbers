pub fn is_armstrong_number(num: u32) -> bool {
    let num_digits = num.to_string().len() as u32;
    let mut result = 0;
    let mut n = num as u64;

    while n > 0 {
        let digit = n % 10;
        result += digit.pow(num_digits);
        n /= 10;
    }

    result == num as u64
}
