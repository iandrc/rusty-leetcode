// Time complexity - O(log n)
// Space complexity - O(1)
impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut steps_counter = 0;

        while num > 0 {
            if num % 2 == 0 {
                num /= 2;
            } else {
                num -= 1;
            }

            steps_counter += 1;
        }

        steps_counter
    }
}

// Bitwise solution
// Time complexity - O(log n)
// Space complexity - O(1)
impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut steps_counter = 0;

        while num > 0 {
            if (num & 1) == 0  {
                num = num >> 1;
            } else {
              num -= 1;
            }

            steps_counter += 1;
        }

        steps_counter
    }
}
