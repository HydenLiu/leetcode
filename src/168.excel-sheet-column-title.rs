/*
 * @lc app=leetcode id=168 lang=rust
 *
 * [168] Excel Sheet Column Title
 *
 * https://leetcode.com/problems/excel-sheet-column-title/description/
 *
 * algorithms
 * Easy (40.53%)
 * Likes:    5488
 * Dislikes: 795
 * Total Accepted:    517.9K
 * Total Submissions: 1.3M
 * Testcase Example:  '1'
 *
 * Given an integer columnNumber, return its corresponding column title as it
 * appears in an Excel sheet.
 *
 * For example:
 *
 *
 * A -> 1
 * B -> 2
 * C -> 3
 * ...
 * Z -> 26
 * AA -> 27
 * AB -> 28
 * ...
 *
 *
 *
 * Example 1:
 *
 *
 * Input: columnNumber = 1
 * Output: "A"
 *
 *
 * Example 2:
 *
 *
 * Input: columnNumber = 28
 * Output: "AB"
 *
 *
 * Example 3:
 *
 *
 * Input: columnNumber = 701
 * Output: "ZY"
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= columnNumber <= 2^31 - 1
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut res = String::new();
        let mut num = column_number;
        while num > 0 {
            num -= 1;
            let char_code = ((num % 26) as u8) + b'A';
            res.insert(0, char_code as char);
            num /= 26
        }
        res
    }
}
// @lc code=end
