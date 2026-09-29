use crate::solution::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_match(s_str: String, p_str: String) -> bool {
        let s = s_str.as_bytes();
        let p = p_str.as_bytes();
        let mut cache = vec![vec![false; p.len() + 1]; s.len() + 1];
        for si in (0..=s.len()).rev() {
            for pi in (0..=p.len()).rev() {
                cache[si][pi] = if pi == p.len() {
                    si == s.len()
                } else if pi < p.len() - 1 && p[pi + 1] == b'*' {
                    cache[si][pi + 2]
                        || (si < s.len() && (p[pi] == b'.' || s[si] == p[pi]) && cache[si + 1][pi])
                } else if si < s.len() && (p[pi] == b'.' || s[si] == p[pi]) {
                    cache[si + 1][pi + 1]
                } else {
                    false
                }
            }
        }
        cache[0][0]
    }

    #[allow(dead_code)]
    pub fn is_match_naive(s: String, p: String) -> bool {
        fn is_match_helper(s: &[u8], p: &[u8], si: usize, pi: usize) -> bool {
            assert!(si <= s.len());
            assert!(pi <= p.len());
            if pi == p.len() {
                return si == s.len();
            }
            if pi < p.len() - 1 && p[pi + 1] == b'*' {
                if is_match_helper(&s, &p, si, pi + 2) {
                    return true;
                }
                if si < s.len()
                    && (p[pi] == b'.' || s[si] == p[pi])
                    && is_match_helper(&s, &p, si + 1, pi)
                {
                    return true;
                }
            } else if si < s.len() && (p[pi] == b'.' || s[si] == p[pi]) {
                if is_match_helper(&s, &p, si + 1, pi + 1) {
                    return true;
                }
            }
            false
        }
        is_match_helper(&s.as_bytes(), &p.as_bytes(), 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match() {
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
        assert_eq!(Solution::is_match("a".to_string(), "ab*".to_string()), true);
        assert_eq!(
            Solution::is_match("aaaaa".to_string(), "a*".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("aaaaab".to_string(), "a*b".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("aaaaac".to_string(), "a*b".to_string()),
            false
        );
        assert_eq!(Solution::is_match("ab".to_string(), "ab".to_string()), true);
        assert_eq!(
            Solution::is_match("ab".to_string(), "ac".to_string()),
            false
        );
        assert_eq!(Solution::is_match("ab".to_string(), "a.".to_string()), true);
        assert_eq!(
            Solution::is_match("abc".to_string(), "a.z".to_string()),
            false
        );
        assert_eq!(
            Solution::is_match("abz".to_string(), "a.z".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("abz".to_string(), ".*".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("abz".to_string(), ".*y".to_string()),
            false
        );
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()),
            false
        );
        assert_eq!(
            Solution::is_match("sip".to_string(), "s*p".to_string()),
            false
        );
        assert_eq!(
            Solution::is_match("sip".to_string(), "s*ip".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("ssssip".to_string(), "s*ip".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("ssssp".to_string(), "s*p".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("missi".to_string(), "mis*i".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("ssi".to_string(), "s*i".to_string()),
            true
        );
    }
}
