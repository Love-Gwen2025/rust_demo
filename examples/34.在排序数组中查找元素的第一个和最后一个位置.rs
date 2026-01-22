/*
 * @lc app=leetcode.cn id=34 lang=rust
 * @lcpr version=30307
 *
 * [34] 在排序数组中查找元素的第一个和最后一个位置
 */

// @lc code=start
impl Solution {
    /// 在排序数组中查找目标值的第一个和最后一个位置
    ///
    /// # 算法思路
    /// 使用两次二分查找：
    /// 1. 第一次找左边界（第一个等于 target 的位置）
    /// 2. 第二次找右边界（最后一个等于 target 的位置）
    ///
    /// # 时间复杂度：O(log n)
    /// # 空间复杂度：O(1)
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 处理空数组的边界情况
        if nums.is_empty() {
            return vec![-1, -1];
        }

        // 分别查找左边界和右边界
        let left = Self::find_left_bound(&nums, target);
        let right = Self::find_right_bound(&nums, target);

        vec![left, right]
    }

    /// 二分查找左边界（第一个等于 target 的位置）
    ///
    /// # 核心思想
    /// 当 nums[mid] == target 时，不立即返回，而是继续向左搜索
    /// 这样可以找到第一个等于 target 的位置
    fn find_left_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0i32;
        let mut right = nums.len() as i32 - 1;

        while left <= right {
            let mut mid = left + (right - left) / 2;
            if nums[mid as usize] == target {
                //找到了，判断是不是左边界，不是就继续搜索，需排除已经搜索过的mid
                if mid == 0 || nums[(mid - 1) as usize] != target {
                    return mid;
                }
                right = mid - 1;
            } else if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        -1 // 未找到目标值
    }

    /// 二分查找右边界（最后一个等于 target 的位置）
    ///
    /// # 核心思想
    /// 当 nums[mid] == target 时，不立即返回，而是继续向右搜索
    /// 这样可以找到最后一个等于 target 的位置
    fn find_right_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0i32;
        let mut right = nums.len() as i32 - 1;
        let n = nums.len() as i32;

        while left <= right {
            // 计算中间位置，避免整数溢出
            let mid = left + (right - left) / 2;

            if nums[mid as usize] == target {
                // 找到目标值，但继续向右搜索，寻找更右边的位置
                // 检查是否是最后一个或者后一个元素不等于 target
                if mid == n - 1 || nums[(mid + 1) as usize] != target {
                    return mid; // 找到右边界
                }
                left = mid + 1; // 继续向右搜索
            } else if nums[mid as usize] < target {
                // 目标在右半部分
                left = mid + 1;
            } else {
                // 目标在左半部分
                right = mid - 1;
            }
        }

         -1 //rust风格，最后一条不写return 未找到目标值
    }
}
// @lc code=end

/*
// @lcpr case=start
// [5,7,7,8,8,10]\n8\n
// @lcpr case=end

// @lcpr case=start
// [5,7,7,8,8,10]\n6\n
// @lcpr case=end

// @lcpr case=start
// []\n0\n
// @lcpr case=end

 */
