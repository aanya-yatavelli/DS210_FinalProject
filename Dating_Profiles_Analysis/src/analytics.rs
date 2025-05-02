//compute cluster size distributions and export the cluster assignments 

use std::error::Error;
use csv::Writer;

/// Compute sizes of each cluster
pub fn cluster_sizes(assignments: &[usize], k: usize) -> Vec<usize> {
    let mut sizes = vec![0; k];
    for &c in assignments {
        if c < k { sizes[c] += 1; }
    }
    sizes
}

/// Write cluster assignments (row index, cluster) to CSV
pub fn write_assignments_csv(assignments: &[usize], path: &str,) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(path)?;
    wtr.write_record(&["row_index", "cluster"])?;
    for (i, &c) in assignments.iter().enumerate() {
        wtr.write_record(&[i.to_string(), c.to_string()])?;
    }
    wtr.flush()?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cluster_sizes_counts() {
        // 7 assignments into 3 clusters: [0,1,1,2,0,2,1]
        let assigns = vec![0, 1, 1, 2, 0, 2, 1];
        let sizes = cluster_sizes(&assigns, 3);
        assert_eq!(sizes, vec![2, 3, 2]); // ← cluster 0 has 2, cluster 1 has 3, cluster 2 has 2
    }
}