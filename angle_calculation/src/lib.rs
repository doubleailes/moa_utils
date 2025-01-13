use std::f64::consts::PI;

/// # MOAngleDropDistance
/// A struct that guaranty the MOA (Minute of Angle) and drop of a bullet given a distance.
///
/// ## Fields
///
/// - The MOA is the angle in degrees that subtends a circle of 1 minute of arc.
/// - The drop is the vertical distance between the point of aim and the point of impact in `m`.
/// - The distance is the distance between the shooter and the target in `m`.
///
pub struct AngleDropDistance {
    angle: AngleType,
    distance: f64,
    drop: f64,
}

const MOA_TO_RAD: f64 = 1000.0 * PI / (180.0 * 60.0);

#[derive(Debug, Clone, Copy)]
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
    pub fn new_from_angle_distance(angle: AngleType, distance: f64) -> Self {
        let drop = distance * angle.get_mrad().tan();
        AngleDropDistance {
            angle,
            distance,
            drop,
        }
    }
    pub fn new_from_drop_distance(drop: f64, distance: f64) -> Self {
        let mrad: f64 = (drop / distance).atan() * 1000.0;
        let angle = AngleType::MIL(mrad);
        AngleDropDistance {
            angle,
            distance,
            drop,
        }
    }
}
