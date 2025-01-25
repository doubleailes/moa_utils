/// # AngleDropDistance
/// A struct that guaranty the MOA (Minute of Angle) and drop of a bullet given a distance.
///
/// ## Fields
///
/// - The **angle** is the angle in degrees that subtends a circle of 1 minute of arc.
/// - The **drop** is the vertical distance between the point of aim and the point of impact in `m`.
/// - The **distance** is the distance between the shooter and the target in `m`.
///
pub struct AngleDropDistance {
    angle: AngleType,
    distance: f64,
    drop: f64,
}

const MOA_TO_RAD: f64 = 0.29088820866572157;

/// # AngleType
/// An enum that represents the type of angle used in the calculation.
///
/// ## Variants
///
/// - **MOA** : The angle is in Minute of Angle. ( 1/60 of a degree )
/// - **MIL** : The angle is in Milliradian. ( 1/1000 of a radian )
///
/// ## Conversion
/// The conversion use the approximation of 1 MOA = 0.29088820866572157 mrad.
///
///
/// - **get_moa** : Convert the angle to MOA.
/// - **get_mrad** : Convert the angle to MIL.
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AngleType {
    MOA(f64),
    MIL(f64),
}

impl AngleType {
    pub fn get_moa(&self) -> f64 {
        match self {
            AngleType::MOA(angle) => *angle,
            AngleType::MIL(angle) => angle / MOA_TO_RAD,
        }
    }
    pub fn get_mrad(&self) -> f64 {
        match self {
            AngleType::MOA(angle) => angle * MOA_TO_RAD,
            AngleType::MIL(angle) => *angle,
        }
    }
}

impl AngleDropDistance {
    pub fn get_angle(&self) -> AngleType {
        self.angle
    }
    pub fn get_distance(&self) -> f64 {
        self.distance
    }
    pub fn get_drop(&self) -> f64 {
        self.drop
    }
    pub fn get_drop_in_cm(&self) -> f64 {
        self.drop * 100.0
    }
    pub fn from_angle_distance(angle: AngleType, distance: f64) -> Self {
        let drop = distance * angle.get_mrad().tan();
        AngleDropDistance {
            angle,
            distance,
            drop,
        }
    }
    pub fn from_drop_distance(drop: f64, distance: f64) -> Self {
        let mrad: f64 = (drop / distance).atan() * 1000.0;
        let angle = AngleType::MIL(mrad);
        AngleDropDistance {
            angle,
            distance,
            drop,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_moa() {
        let angle_moa = AngleType::MOA(1.0);
        let angle_mil = AngleType::MIL(1.0);
        assert_eq!(angle_moa.get_moa(), 1.0);
        assert!((angle_mil.get_moa() - (1.0 / MOA_TO_RAD)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_get_mrad() {
        let angle_moa = AngleType::MOA(1.0);
        let angle_mil = AngleType::MIL(1.0);
        assert!((angle_moa.get_mrad() - (1.0 * MOA_TO_RAD)).abs() < f64::EPSILON);
        assert_eq!(angle_mil.get_mrad(), 1.0);
    }

    #[test]
    fn test_from_angle_distance() {
        let angle = AngleType::MOA(1.0);
        let distance = 100.0;
        let add = AngleDropDistance::from_angle_distance(angle, distance);
        assert_eq!(add.get_angle(), angle);
        assert_eq!(add.get_distance(), distance);
        assert!((add.get_drop() - (distance * angle.get_mrad().tan())).abs() < f64::EPSILON);
    }

    #[test]
    fn test_from_drop_distance() {
        let drop = 1.0;
        let distance = 100.0;
        let add = AngleDropDistance::from_drop_distance(drop, distance);
        let expected_angle = AngleType::MIL((drop / distance).atan() * 1000.0);
        assert_eq!(add.get_angle(), expected_angle);
        assert_eq!(add.get_distance(), distance);
        assert_eq!(add.get_drop(), drop);
    }

    #[test]
    fn test_get_drop_in_cm() {
        let angle = AngleType::MOA(1.0);
        let distance = 100.0;
        let add = AngleDropDistance::from_angle_distance(angle, distance);
        assert_eq!(add.get_drop_in_cm(), add.get_drop() * 100.0);
    }
}
