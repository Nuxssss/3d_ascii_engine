use crate::point3d::Point3D;

pub struct Cube {
    pub coordinate: Point3D,
    pub size: f32,
}

impl Cube {
    pub fn plot(&self) -> [Point3D; 8] {
        [
            Point3D::new(
                self.coordinate.x,
                self.coordinate.y,
                self.coordinate.z,
            ),
            Point3D::new(
                self.coordinate.x + self.size,
                self.coordinate.y,
                self.coordinate.z,
            ),
            Point3D::new(
                self.coordinate.x,
                self.coordinate.y + self.size,
                self.coordinate.z,
            ),
            Point3D::new(
                self.coordinate.x,
                self.coordinate.y,
                self.coordinate.z + self.size,
            ),
            Point3D::new(
                self.coordinate.x + self.size,
                self.coordinate.y + self.size,
                self.coordinate.z,
            ),
            Point3D::new(
                self.coordinate.x,
                self.coordinate.y + self.size,
                self.coordinate.z + self.size,
            ),
            Point3D::new(
                self.coordinate.x + self.size,
                self.coordinate.y,
                self.coordinate.z + self.size,
            ),
            Point3D::new(
                self.coordinate.x + self.size,
                self.coordinate.y + self.size,
                self.coordinate.z + self.size,
            ),
        ]
    }
}
