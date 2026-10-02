struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn count_texts_naive(pressed_keys: String) -> i32 {
        fn helper(keys: &[u8], index: usize) -> i32 {
            if index == keys.len() {
                1
            } else {
                let mut count = helper(keys, index + 1);
                match keys[index] {
                    b'2' | b'3' | b'4' | b'5' | b'6' | b'8' => {
                        if index + 1 < keys.len() && keys[index + 1] == keys[index] {
                            count += helper(keys, index + 2);
                            if index + 2 < keys.len() && keys[index + 2] == keys[index] {
                                count += helper(keys, index + 3);
                            }
                        }
                    }
                    b'7' | b'9' => {
                        if index + 1 < keys.len() && keys[index + 1] == keys[index] {
                            count += helper(keys, index + 2);
                            if index + 2 < keys.len() && keys[index + 2] == keys[index] {
                                count += helper(keys, index + 3);
                                if index + 3 < keys.len() && keys[index + 3] == keys[index] {
                                    count += helper(keys, index + 4);
                                }
                            }
                        }
                    }
                    _ => {
                        assert!(false);
                    }
                }
                count
            }
        }
        helper(pressed_keys.as_bytes(), 0)
    }

    #[allow(dead_code)]
    pub fn count_texts(pressed_keys_str: String) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let keys = pressed_keys_str.as_bytes();
        let mut cache = vec![0; keys.len() + 1];
        cache[keys.len()] = 1;
        for (index, &c) in keys.iter().enumerate().rev() {
            cache[index] = {
                let mut count = cache[index + 1];
                if index + 1 < keys.len() && keys[index + 1] == c {
                    count = (count + cache[index + 2]) % MOD;
                    if index + 2 < keys.len() && keys[index + 2] == c {
                        count = (count + cache[index + 3]) % MOD;
                        if (c == b'7' || c == b'9')
                            && index + 3 < keys.len()
                            && keys[index + 3] == c
                        {
                            count = (count + cache[index + 4]) % MOD;
                        }
                    }
                }
                count
            }
        }
        cache[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gest_generate_parenthesis() {
        assert_eq!(Solution::count_texts("22233".to_string()), 8);
        assert_eq!(
            Solution::count_texts("222222222222222222222222222222222222".to_string()),
            82876089
        );
    }
}
