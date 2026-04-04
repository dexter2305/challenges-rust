pub fn number_of_steps(num: i32) -> i32 {
    let mut counter = 0;
    let mut n = num;
    while n != 0 {
        n = if n % 2 == 0 { n / 2 } else { n - 1 };
        counter += 1;
    }
    counter
}

pub fn number_of_steps_bitwise(num: i32) -> i32 {
    let mut n = num;
    let mut counter = 0;
    while n != 0 {
        n = if n & 1 == 1 { n - 1 } else { n >> 1 };
        counter += 1;
    }
    counter
}

#[cfg(test)]
mod tests {
    use crate::leetcode::p1342;
    #[test]
    fn number_of_steps() {
        let tests = vec![(14, 6), (8, 4)];

        for (n, exp) in &tests {
            assert_eq!(p1342::number_of_steps(*n), *exp, "Given input = {}", *n)
        }

        for (n, exp) in &tests {
            assert_eq!(
                p1342::number_of_steps_bitwise(*n),
                *exp,
                "Given input = {}",
                *n
            )
        }
    }
}
