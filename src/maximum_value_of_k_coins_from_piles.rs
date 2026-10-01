// recursion:
//
// fn solution(pile_index: i32, coins_left: i32) {
//     if pile_index == piles.len() or coins_left == 0 {
//         0
//     } else {
//         let mut pile_value = 0;
//         let mut best = solution(pile_index + 1, coins_left); // skip this pile
//         for i in 0..pile[pile_index].len() {
//             if coins_left < (i + 1) {
//                 break;
//             }
//             pile_value += pile[pile_index][i];
//             best = best.max(pile_value + solution(pile_index + 1, coins_left - (i + 1)));
//         }
//         best
//     }
// }

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut cache = vec![vec![0; k as usize + 1]; piles.len() as usize + 1];
        for pile_index in (0usize..piles.len()).rev() {
            for coins_left in 0usize..=(k as usize) {
                let mut pile_value = 0;
                let mut best = cache[pile_index + 1][coins_left]; // skip this pile
                for (i, &c) in piles[pile_index].iter().enumerate() {
                    if coins_left < i + 1 {
                        break;
                    }
                    pile_value += c;
                    best = best.max(pile_value + cache[pile_index + 1][coins_left - (i + 1)]);
                }
                cache[pile_index][coins_left] = best;
            }
        }
        cache[0][k as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_value_of_coins() {
        assert_eq!(
            Solution::max_value_of_coins(vec![vec![1, 100, 3], vec![7, 8, 9]], 2),
            101
        );
    }
}
