/// K-MC on a set of vectors
pub fn kmeans(
    data: &[Vec<f64>],
    k: usize,
    max_iter: usize,
) -> (Vec<Vec<f64>>, Vec<usize>) {
    let n = data.len();
    let dim = if n > 0 { data[0].len() } else { 0 };
    let mut centroids = data.iter().take(k).cloned().collect::<Vec<_>>();
    let mut assignments = vec![0; n];

    for _ in 0..max_iter {
        let mut moved = false;

        // assignment step
        for i in 0..n {
            let (best, _) = centroids.iter().enumerate().map(|(c, cen)| {
                let dist: f64 = data[i].iter().zip(cen).map(|(a,b)| (a-b).powi(2)).sum();
                (c, dist)
            }).min_by(|x,y| x.1.partial_cmp(&y.1).unwrap()).unwrap();
            if assignments[i] != best {
                assignments[i] = best;
                moved = true;
            }
        }
        if !moved { break; }

        // update step
        let mut counts = vec![0; k];
        let mut sums = vec![vec![0.0; dim]; k];
        for i in 0..n {
            let c = assignments[i];
            counts[c] += 1;
            for j in 0..dim {
                sums[c][j] += data[i][j];
            }
        }
        for c in 0..k {
            if counts[c] > 0 {
                for j in 0..dim {
                    centroids[c][j] = sums[c][j] / counts[c] as f64;
                }
            }
        }
    }
    (centroids, assignments)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_kmeans() {
        let data = vec![vec![0.0], vec![1.0]];
        let (_c, assigns) = kmeans(&data, 2, 5);
        assert_eq!(assigns, vec![0, 1]);
    }
}