// Time complexity - O(n)
// Space complexity - O(1)
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut prev: i32 = 0;

        for num in nums {
            res.push(num + prev);
            prev += num;
        }
        res
    }
}
