/*
 * @lc app=leetcode id=171 lang=rust
 *
 * [171] Excel Sheet Column Number
 *
 * https://leetcode.com/problems/excel-sheet-column-number/description/
 *
 * algorithms
 * Easy (63.65%)
 * Likes:    4709
 * Dislikes: 365
 * Total Accepted:    679.1K
 * Total Submissions: 1.1M
 * Testcase Example:  '"A"'
 *
 * Given a string columnTitle that represents the column title as appears in an
 * Excel sheet, return its corresponding column number.
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
 * Input: columnTitle = "A"
 * Output: 1
 *
 *
 * Example 2:
 *
 *
 * Input: columnTitle = "AB"
 * Output: 28
 *
 *
 * Example 3:
 *
 *
 * Input: columnTitle = "ZY"
 * Output: 701
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= columnTitle.length <= 7
 * columnTitle consists only of uppercase English letters.
 * columnTitle is in the range ["A", "FXSHRXW"].
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        column_title
            .chars()
            .map(|t| t as i32 - 'A' as i32 + 1)
            .fold(0, |p, n| p * 26 + n)
    }
}
// @lc code=end
