struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn num_decodings(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut cache = vec![0; bytes.len() + 1];
        cache[bytes.len()] = 1;
        for (index, &c) in bytes.iter().enumerate().rev() {
            cache[index] = match c {
                b'0' => 0,
                b'1' => {
                    let mut count = cache[index + 1];
                    if index < bytes.len() - 1 {
                        count += cache[index + 2];
                    }
                    count
                }
                b'2' => {
                    let mut count = cache[index + 1];
                    if index < bytes.len() - 1 && bytes[index + 1] <= b'6' {
                        count += cache[index + 2];
                    }
                    count
                }
                _ => cache[index + 1],
            }
        }
        cache[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_decodings() {
        assert_eq!(Solution::num_decodings("12".to_string()), 2);
        assert_eq!(Solution::num_decodings("06".to_string()), 0);
        assert_eq!(Solution::num_decodings("60".to_string()), 0);
        assert_eq!(Solution::num_decodings("61".to_string()), 1);
        assert_eq!(Solution::num_decodings("10".to_string()), 1);
        assert_eq!(Solution::num_decodings("11106".to_string()), 2);
    }
}
