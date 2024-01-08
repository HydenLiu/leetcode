/*
 * @lc app=leetcode id=67 lang=rust
 *
 * [67] Add Binary
 *
 * https://leetcode.com/problems/add-binary/description/
 *
 * algorithms
 * Easy (53.00%)
 * Likes:    9068
 * Dislikes: 919
 * Total Accepted:    1.3M
 * Total Submissions: 2.5M
 * Testcase Example:  '"11"\n"1"'
 *
 * Given two binary strings a and b, return their sum as a binary string.
 *
 *
 * Example 1:
 * Input: a = "11", b = "1"
 * Output: "100"
 * Example 2:
 * Input: a = "1010", b = "1011"
 * Output: "10101"
 *
 *
 * Constraints:
 *
 *
 * 1 <= a.length, b.length <= 10^4
 * a and b consist only of '0' or '1' characters.
 * Each string does not contain leading zeros except for the zero itself.
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (mut a, mut b) = (a.chars().collect::<Vec<_>>(), b.chars().collect::<Vec<_>>());
        let (mut i, mut j, mut carry, mut res) =
            (a.len() as i32 - 1, b.len() as i32 - 1, 0, String::new());

        while i >= 0 || j >= 0 || carry > 0 {
            let mut sum = carry;
            if i >= 0 {
                sum += a[i as usize].to_digit(10).unwrap();
                i -= 1;
            }
            if j >= 0 {
                sum += b[j as usize].to_digit(10).unwrap();
                j -= 1;
            }

            carry = sum / 2;
            res.insert(0, std::char::from_digit(sum % 2, 10).unwrap());
        }
        res
    }
}
// @lc code=end
