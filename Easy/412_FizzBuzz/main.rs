// Time complexity - O(n)
// Space complexity - O(1)
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result: Vec<String> = vec![1.to_string()];

        for i in 2..=n {
            if i % 3 == 0 {
                if i % 5 == 0 {
                    result.push("FizzBuzz".to_string());
                } else {
                    result.push("Fizz".to_string());
                }
            } else if i % 5 == 0 {
                result.push("Buzz".to_string());
            } else {
                result.push(i.to_string());
            }
        }

        result
    }
}
