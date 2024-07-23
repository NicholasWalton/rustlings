fn factorial(num: u8) -> u64 {
    // match num {
    //     0 => 1,
    //     num => factorial(num - 1) * u64::from(num),
    // }
    // (2..=num).fold(1, |acc, x|  {acc * u64::from(x)})
    (2..=u64::from(num)).product()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
