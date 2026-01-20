/// LeetCode 17: 电话号码的字母组合
/// 给定一个仅包含数字 2-9 的字符串，返回所有它能表示的字母组合。
///
/// 核心思路：回溯算法 (Backtracking)
/// 1. 建立数字到字母的映射表
/// 2. 逐个处理输入的数字，尝试每个可能的字母
/// 3. 递归深入，到达末尾时收集结果

fn letter_combinations(digits: String) -> Vec<String> {
    // 边界情况：空输入直接返回空结果
    if digits.is_empty() {
        return vec![];
    }

    // 数字 2-9 对应的字母映射（索引 0-7 对应数字 2-9）
    let phone_map: [&str; 8] = [
        "abc",  // 2
        "def",  // 3
        "ghi",  // 4
        "jkl",  // 5
        "mno",  // 6
        "pqrs", // 7
        "tuv",  // 8
        "wxyz", // 9
    ];

    let mut results: Vec<String> = Vec::new();
    let digits_chars: Vec<char> = digits.chars().collect();

    // 启动回溯
    backtrack(&phone_map, &digits_chars, 0, String::new(), &mut results);

    results
}

/// 回溯函数
/// - phone_map: 数字到字母的映射
/// - digits: 输入的数字序列
/// - index: 当前处理到第几个数字
/// - current: 当前已组合的字符串
/// - results: 存放最终结果
fn backtrack(
    phone_map: &[&str; 8],
    digits: &[char],
    index: usize,
    current: String,
    results: &mut Vec<String>,
) {
    // 递归终止条件：已处理完所有数字
    if index == digits.len() {
        results.push(current);
        return;
    }

    // 获取当前数字对应的字母列表
    // 数字 '2' 的 ASCII 值是 50，减去 50 得到索引 0
    let digit = digits[index];
    let letters = phone_map[(digit as usize) - ('2' as usize)];

    // 遍历当前数字的每个可能字母
    for letter in letters.chars() {
        // 选择当前字母，递归处理下一个数字
        let mut next = current.clone();
        next.push(letter);
        backtrack(phone_map, digits, index + 1, next, results);
        // 回溯：这里不需要显式撤销，因为 `next` 是 clone 出来的新字符串
    }
}

fn main() {
    // 测试用例
    let test_cases = vec!["23", "2", "79", ""];

    for digits in test_cases {
        let result = letter_combinations(digits.to_string());
        println!("输入: \"{}\"", digits);
        println!("输出: {:?}", result);
        println!("组合数量: {}", result.len());
        println!("---");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // LeetCode 官方示例
        let result = letter_combinations("23".to_string());
        assert_eq!(
            result,
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }

    #[test]
    fn test_empty_input() {
        let result = letter_combinations("".to_string());
        assert!(result.is_empty());
    }

    #[test]
    fn test_single_digit() {
        let result = letter_combinations("2".to_string());
        assert_eq!(result, vec!["a", "b", "c"]);
    }
}
