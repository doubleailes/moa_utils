use std::f64::consts::PI;

/// # MOADD
/// A struct that guaranty the MOA (Minute of Angle) and drop of a bullet given a distance.
///
/// ## Fields
///
/// - The MOA is the angle in degrees that subtends a circle of 1 minute of arc.
/// - The drop is the vertical distance between the point of aim and the point of impact in `m`.
/// - The distance is the distance between the shooter and the target in `m`.
///
/// # Examples
/// ```
/// use angle_calculation::MOADD;
/// let computed_drop: f64 = MOADD::new_from_moa_distance(0.2, 100.0).get_drop_in_cm();
/// assert_eq!(computed_drop, 0.5817764179878108);
/// let computed_moa: f64 = MOADD::new_from_drop_distance(0.05, 100.0).get_moa();
/// assert_eq!(computed_moa, 1.7188732421530424);
/// ```
pub struct MOADD {
    moa: f64,
    distance: f64,
    drop: f64,
}

impl MOADD {
    /// Get the moa value
    pub fn get_moa(&self) -> f64 {
        self.moa
    }
    /// Get the drop value in `m`
    pub fn get_drop(&self) -> f64 {
        self.drop
    }
    /// Get the drop value in `cm`
    pub fn get_drop_in_cm(&self) -> f64 {
        self.drop * 100.0
    }
    /// Get the distance value in `m`
    pub fn get_distance(&self) -> f64 {
        self.distance
    }
    /// Create a new MOADD from a given moa and distance (both in `m`).
    pub fn new_from_moa_distance(moa: f64, distance: f64) -> Self {
        let moa_in_radians: f64 = (PI / 180.0) * (moa / 60.0);
        let drop: f64 = distance * moa_in_radians.tan();
        MOADD {
            moa,
            distance,
            drop,
        }
    }
    /// Create a new MOADD from a given drop and distance ( both in `m`).
    pub fn new_from_drop_distance(drop: f64, distance: f64) -> Self {
        let moa: f64 = (drop / distance).atan() * (180.0 / PI) * 60.0;
        MOADD {
            moa,
            distance,
            drop,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_inverse_moa() {
        let computed_drop: f64 = MOADD::new_from_moa_distance(0.2, 100.0).get_drop_in_cm();
        assert_eq!(computed_drop, 0.5817764179878108);
    }

    #[test]
    fn test_calculate_size_from_moa() {
        let computed_moa: f64 = MOADD::new_from_drop_distance(0.05, 100.0).get_moa();
        assert_eq!(computed_moa, 1.7188732421530424);
    }
}
