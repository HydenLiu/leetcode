/*
 * @lc app=leetcode id=69 lang=rust
 *
 * [69] Sqrt(x)
 *
 * https://leetcode.com/problems/sqrtx/description/
 *
 * algorithms
 * Easy (38.25%)
 * Likes:    7756
 * Dislikes: 4418
 * Total Accepted:    1.8M
 * Total Submissions: 4.6M
 * Testcase Example:  '4'
 *
 * Given a non-negative integer x, return the square root of x rounded down to
 * the nearest integer. The returned integer should be non-negative as well.
 *
 * You must not use any built-in exponent function or operator.
 *
 *
 * For example, do not use pow(x, 0.5) in c++ or x ** 0.5 in python.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: x = 4
 * Output: 2
 * Explanation: The square root of 4 is 2, so we return 2.
 *
 *
 * Example 2:
 *
 *
 * Input: x = 8
 * Output: 2
 * Explanation: The square root of 8 is 2.82842..., and since we round it down
 * to the nearest integer, 2 is returned.
 *
 *
 *
 * Constraints:
 *
 *
 * 0 <= x <= 2^31 - 1
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let targer = x as i64;
        let mut upper: i64 = 1 << 16;
        let mut lower: i64 = 0;

        while upper >= lower {
            let mid = lower + (upper - lower) / 2;
            if mid * mid > target {
                upper = mid - 1;
            } else {
                lower = mid + 1;
            }
        }

        upper as i32
    }
}
// @lc code=end
