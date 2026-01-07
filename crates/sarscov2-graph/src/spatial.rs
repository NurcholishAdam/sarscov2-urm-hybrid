//! 3D spatial operations for protein structures

use crate::nodes::Coordinate3D;

/// Calculate Euclidean distance between two 3D points
pub fn distance(p1: &Coordinate3D, p2: &Coordinate3D) -> f64 {
    p1.distance_to(p2)
}

/// Find nearest neighbors within radius
pub fn find_neighbors(
    center: &Coordinate3D,
    points: &[Coordinate3D],
    radius: f64,
) -> Vec<usize> {
    points
        .iter()
        .enumerate()
        .filter(|(_, p)| distance(center, p) <= radius)
        .map(|(i, _)| i)
        .collect()
}

/// Calculate center of mass for a set of coordinates
pub fn center_of_mass(points: &[Coordinate3D]) -> Option<Coordinate3D> {
    if points.is_empty() {
        return None;
    }

    let sum_x: f64 = points.iter().map(|p| p.x).sum();
    let sum_y: f64 = points.iter().map(|p| p.y).sum();
    let sum_z: f64 = points.iter().map(|p| p.z).sum();
    let n = points.len() as f64;

    Some(Coordinate3D::new(sum_x / n, sum_y / n, sum_z / n))
}

/// Calculate RMSD (Root Mean Square Deviation) between two structures
pub fn rmsd(structure1: &[Coordinate3D], structure2: &[Coordinate3D]) -> Option<f64> {
    if structure1.len() != structure2.len() || structure1.is_empty() {
        return None;
    }

    let sum_sq_dist: f64 = structure1
        .iter()
        .zip(structure2.iter())
        .map(|(p1, p2)| {
            let d = distance(p1, p2);
            d * d
        })
        .sum();

    Some((sum_sq_dist / structure1.len() as f64).sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let p1 = Coordinate3D::new(0.0, 0.0, 0.0);
        let p2 = Coordinate3D::new(1.0, 0.0, 0.0);
        assert_eq!(distance(&p1, &p2), 1.0);
    }

    #[test]
    fn test_center_of_mass() {
        let points = vec![
            Coordinate3D::new(0.0, 0.0, 0.0),
            Coordinate3D::new(2.0, 0.0, 0.0),
        ];
        let center = center_of_mass(&points).unwrap();
        assert_eq!(center.x, 1.0);
        assert_eq!(center.y, 0.0);
        assert_eq!(center.z, 0.0);
    }

    #[test]
    fn test_find_neighbors() {
        let center = Coordinate3D::new(0.0, 0.0, 0.0);
        let points = vec![
            Coordinate3D::new(0.5, 0.0, 0.0),
            Coordinate3D::new(2.0, 0.0, 0.0),
            Coordinate3D::new(0.3, 0.0, 0.0),
        ];
        let neighbors = find_neighbors(&center, &points, 1.0);
        assert_eq!(neighbors.len(), 2);
    }
}
