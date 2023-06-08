use std::ops;
#[derive(Default, Debug, Clone, Copy)]
pub struct FVec {
    pub x: f64,
    pub y: f64,
}

impl FVec {
    pub const fn new(x: f64, y: f64) -> Self {
        FVec { x, y }
    }

    pub fn project_on(&self, other: &FVec) -> FVec {
        let (v1, v2) = (self.clone(), other.clone());
        if v2.x == 0.0 && v2.y == 0.0 {
            return FVec{
                x: 0.0,
                y: 0.0,
            }
        }
        let dot = v1 * v2;
        let mag_sq = v2 * v2;
        let scalar = dot / mag_sq;
        return FVec{
            x: scalar * v2.x,
            y: scalar * v2.y,
        }
    }

    pub fn distance(&self, other: &FVec) -> f64 {
        let vec = self.clone() - other.clone();
        (vec.x.powi(2) + vec.y.powi(2)).sqrt()
    }
}

impl ops::Sub<FVec> for FVec {
    type Output = FVec;
    fn sub(self, rhs: FVec) -> Self::Output {
        FVec {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Div<FVec> for FVec {
    type Output = FVec;
    fn div(self, rhs: FVec) -> Self {
        if rhs.x == 0.0 || rhs.y == 0.0 {
            return self;
        }
        FVec {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl ops::Add<FVec> for FVec {
    type Output = FVec;
    fn add(self, rhs: FVec) -> Self {
        if rhs.x == 0.0 || rhs.y == 0.0 {
            return self;
        }
        FVec {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Mul<FVec> for FVec {
    type Output = f64;
    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl ops::Mul<f64> for FVec {
    type Output = FVec;
    fn mul(self, rhs: f64) -> Self::Output {
        FVec{
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_on_same_vec() {
        let (v1, v2) = (FVec::new(0.0, 1.0), FVec::new(0.0, 1.0));
        let projection = v1.project_on(&v2);
        assert_eq!(projection.x, 0.0, "should equal");
        assert_eq!(projection.y, 1.0, "should equal");
    }

    #[test]
    fn test_project_on_0_2_0_1() {
        let (v1, v2) = (FVec::new(0.0, 2.0), FVec::new(0.0, 1.0));
        let projection = v1.project_on(&v2);
        assert_eq!(projection.x, 0.0, "should equal");
        assert_eq!(projection.y, 2.0, "should equal");
    }

    #[test]
    fn test_project_on_0_2_1_0() {
        let (v1, v2) = (FVec::new(0.0, 2.0), FVec::new(1.0, 0.0));
        let projection = v1.project_on(&v2);
        assert_eq!(projection.x, 0.0, "should equal");
        assert_eq!(projection.y, 0.0, "should equal");
    }

    #[test]
    fn test_project_on_1_1_1_0() {
        let (v1, v2) = (FVec::new(1.0, 1.0), FVec::new(1.0, 0.0));
        let mut projection = v1.project_on(&v2);
        assert_eq!(projection.x, 1.0, "should equal");
        assert_eq!(projection.y, 0.0, "should equal");

        projection = v2.project_on(&v1);
        assert_eq!(projection.x, 0.5, "should equal");
        assert_eq!(projection.y, 0.5, "should equal");
    }

    #[test]
    fn test_project_on_opposite_dir() {
        let (v1, v2) = (FVec::new(1.0, 1.0), FVec::new(-1.0, 0.0));
        let projection = v1.project_on(&v2);
        assert_eq!(projection.x, 1.0, "should equal");
        assert_eq!(projection.y, 0.0, "should equal");
    }
}
