pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn add_binary(a: String, b: String) -> String {
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();
        let mut result: Vec<u8> = vec![];
        let mut i = a.len() as i32 - 1;
        let mut j = b.len() as i32 - 1;
        let mut c = 0;
        while i >= 0 || j >= 0 {
            let x = if i >= 0 {
                (a_bytes[i as usize] - b'0') as i32
            } else {
                0
            };
            let y = if j >= 0 {
                (b_bytes[j as usize] - b'0') as i32
            } else {
                0
            };
            let s = x + y + c;
            result.push((s & 1) as u8 + b'0');
            c = s >> 1;
            i -= 1;
            j -= 1;
        }
        if c > 0 {
            result.push(c as u8 + b'0');
        }
        result.reverse();
        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "1".to_string()),
            "100".to_string()
        );
        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );
    }
}
