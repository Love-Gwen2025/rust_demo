/*
 * @lc app=leetcode.cn id=35 lang=rust
 * @lcpr version=30307
 *
 * [35] 搜索插入位置
 */

// @lc code=start
impl Solution {
    /// 搜索插入位置
    ///
    /// 在有序数组中找到目标值的位置，如果不存在则返回应该插入的位置
    ///
    /// # 时间复杂度：O(log n)
    /// # 空间复杂度：O(1)
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0i32;
        let mut right = nums.len() as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;

            if nums[mid as usize] == target {
                return mid; // 找到目标，返回索引
            } else if nums[mid as usize] < target {
                left = mid + 1; // 目标在右半部分
            } else {
                right = mid - 1; // 目标在左半部分
            }
        }

        // 循环结束，left 就是插入位置
        // 此时 left > right，目标值应该插入到 left 的位置
        left
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,3,5,6]\n5\n
// @lcpr case=end

// @lcpr case=start
// [1,3,5,6]\n2\n
// @lcpr case=end

// @lcpr case=start
// [1,3,5,6]\n7\n
// @lcpr case=end

 */
