struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn read_binary_watch(turned_in: i32) -> Vec<String> {
        let mut results = Vec::new();
        for i in 0i32..(1 << 10) {
            if i.count_ones() == turned_in as u32 {
                let hour = i >> 6;
                let minutes = i & ((1 << 6) - 1);
                if hour < 12 && minutes < 60 {
                    results.push(format!("{}:{:02}", hour, minutes));
                }
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gest_generate_parenthesis() {
        assert_eq!(
            Solution::read_binary_watch(1),
            vec!["0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00"]
        );
    }
}
