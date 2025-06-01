/// Trait to extract the x, y and z values
pub trait GetXYZ {
    /// Returns the x value
    fn x(&self) -> f64;

    /// Returns the y value
    fn y(&self) -> f64;

    /// Returns the z value if the type has one
    fn z(&self) -> Option<f64>;
}

impl GetXYZ for (f64, f64, f64) {
    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }

    fn z(&self) -> Option<f64> {
        Some(self.2)
    }
}

impl GetXYZ for (f64, f64) {
    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }

    fn z(&self) -> Option<f64> {
        None
    }
}

impl GetXYZ for [f64; 3] {
    fn x(&self) -> f64 {
        self[0]
    }

    fn y(&self) -> f64 {
        self[1]
    }

    fn z(&self) -> Option<f64> {
        Some(self[2])
    }
}

impl GetXYZ for [f64; 2] {
    fn x(&self) -> f64 {
        self[0]
    }

    fn y(&self) -> f64 {
        self[1]
    }

    fn z(&self) -> Option<f64> {
        None
    }
}

#[cfg(feature = "geo-3d-types")]
impl GetXYZ for geo_3d_types::Coord {
    fn x(&self) -> f64 {
        self.z
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> Option<f64> {
        Some(self.z)
    }
}
