/*
 * @lc app=leetcode.cn id=3507 lang=rust
 * @lcpr version=30307
 *
 * [3507] 移除最小数对使数组有序 I
 */

impl Solution {
    /// 移除最小数对使数组有序
    ///
    /// # 算法思路
    /// 直接模拟：每次找到和最小的相邻对，合并它们，直到数组非递减
    ///
    /// # 时间复杂度：O(n^3) - 最多 n 次操作，每次需要 O(n) 检查和 O(n) 合并
    /// # 空间复杂度：O(1) - 原地修改（使用可变引用）
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        // 转为可变向量，因为需要修改
        let mut nums = nums;
        let mut count = 0;

        // 循环直到数组变为非递减
        while !Self::is_non_decreasing(&nums) {
            // 找到和最小的相邻对的索引
            let min_idx = Self::find_min_sum_pair_index(&nums);

            // 合并这对元素：用它们的和替换
            let sum = nums[min_idx] + nums[min_idx + 1];
            nums[min_idx] = sum; // 替换第一个元素为和
            nums.remove(min_idx + 1); // 移除第二个元素

            count += 1;
        }

        count
    }

    /// 检查数组是否非递减（即每个元素 >= 前一个元素）
    fn is_non_decreasing(nums: &Vec<i32>) -> bool {
        // 遍历检查：如果任意 nums[i] > nums[i+1]，则不是非递减
        for i in 0..nums.len() - 1 {
            if nums[i] > nums[i + 1] {
                return false; // 发现递减，返回 false
            }
        }
        true // 全部检查通过，是非递减
    }

    /// 找到和最小的相邻对的索引（如果有多个，返回最左边的）
    fn find_min_sum_pair_index(nums: &Vec<i32>) -> usize {
        let mut min_sum = i32::MAX; // 初始化为最大值
        let mut min_idx = 0;

        // 遍历所有相邻对
        for i in 0..nums.len() - 1 {
            let sum = nums[i] + nums[i + 1];
            if sum < min_sum {
                min_sum = sum;
                min_idx = i;
            }
            // 注意：如果 sum == min_sum，我们不更新 min_idx
            // 这样自动保留了"最左边的"
        }

        min_idx
    }
}
// @lc code=end

/*
// @lcpr case=start
// [5,2,3,1]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,2]\n
// @lcpr case=end

 */
