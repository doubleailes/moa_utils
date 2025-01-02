/// # MRADDD
/// A struct that guaranty the MRAD (Milliradian) and drop of a bullet given a distance.
///
/// ## Fields
///
/// - The MRAD is the angle in radians that subtends a circle of 1 milliradian.
/// - The drop is the vertical distance between the point of aim and the point of impact in `m`.
/// - The distance is the distance between the shooter and the target in `m`.
///
/// # Examples
/// ```
/// use angle_calculation::MRADDD;
/// let computed_drop: f64 = MRADDD::new_from_mrad_distance(1.0, 100.0).get_drop_in_cm();
/// assert_eq!(computed_drop, 10.000003333334668);
/// let computed_mrad: f64 = MRADDD::new_from_drop_distance(0.1, 100.0).get_mrad();
/// assert_eq!(computed_mrad, 0.9999996666668668);
/// ```
pub struct MRADDD {
    mrad: f64,
    distance: f64,
    drop: f64,
}

impl MRADDD {
    pub fn get_mrad(&self) -> f64 {
        self.mrad
    }
    pub fn get_drop(&self) -> f64 {
        self.drop
    }
    pub fn get_drop_in_cm(&self) -> f64 {
        self.drop * 100.0
    }
    pub fn get_distance(&self) -> f64 {
        self.distance
    }
    pub fn new_from_mrad_distance(mrad: f64, distance: f64) -> Self {
        let drop = distance * (mrad / 1000.0).tan();
        MRADDD {
            mrad,
            distance,
            drop,
        }
    }
    pub fn new_from_drop_distance(drop: f64, distance: f64) -> Self {
        let mrad: f64 = (drop / distance).atan() * 1000.0;
        MRADDD {
            mrad,
            distance,
            drop,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_inverse_mrad() {
        let computed_drop: f64 = MRADDD::new_from_mrad_distance(1.0, 100.0).get_drop_in_cm();
        assert_eq!(computed_drop, 10.000003333334668);
    }

    #[test]
    fn test_calculate_mrad() {
        let computed_mrad: f64 = MRADDD::new_from_drop_distance(0.1, 100.0).get_mrad();
        assert_eq!(computed_mrad, 0.9999996666668668);
    }
}
