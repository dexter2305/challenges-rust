/// 412. Fizz Buzz

pub fn fizz_buzz(n: i32) -> Vec<String> {
    (1..=n)
        .into_iter()
        .map(|e| match (e % 3, e % 5) {
            (0, 0) => "FizzBuzz".to_owned(),
            (0, _) => "Fizz".to_owned(),
            (_, 0) => "Buzz".to_owned(),
            (_, _) => e.to_string(),
        })
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {

    use crate::leetcode::p0412;

    #[test]
    fn fizz_buzz() {
        let tests = vec![
            (1, vec!["1"]),
            (3, vec!["1", "2", "Fizz"]),
            (5, vec!["1", "2", "Fizz", "4", "Buzz"]),
            (
                15,
                vec![
                    "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                    "13", "14", "FizzBuzz",
                ],
            ),
        ];

        for (n, exp) in tests {
            assert_eq!(
                p0412::fizz_buzz(n),
                exp.iter().map(|e| e.to_string()).collect::<Vec<String>>()
            )
        }
    }
}
