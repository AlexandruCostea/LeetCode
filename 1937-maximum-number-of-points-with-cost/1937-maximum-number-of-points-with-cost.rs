use std::cmp::max;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let mut results: Vec<i64> = Vec::new();
        for i in 0..points[0].len() {
            results.push(points[0][i] as i64);
        }

        for i in 1..points.len() {
            let mut left_max: Vec<i64> = vec![0; points[i].len()];
            let mut right_max: Vec<i64> = vec![0; points[i].len()];

            left_max[0] = results[0];
            for j in 1..results.len() {
                left_max[j] = max(left_max[j-1] - 1, results[j]);
            }

            right_max[results.len() - 1] = results[results.len() - 1];
            for j in (0..right_max.len() - 1).rev() {
                right_max[j] = max(right_max[j+1] - 1, results[j]);
            }

            for j in 0..results.len() {
                results[j] = max(left_max[j], right_max[j]) + points[i][j] as i64;
            }
        }

        let mut maxi = 0;
        for elem in &results {
            maxi = max(maxi, *elem);
        }
        return maxi;
    }
}