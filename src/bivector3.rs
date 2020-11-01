#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Bivector3D {
    yz: f32,
    xz: f32,
    xy: f32,
}

impl Bivector3D {
    fn vector_mul(self, rhs: Self) -> Self {
        Bivector3D {
            yz: self.xz * rhs.xy - self.xy * rhs.xz,
            xz: self.xy * rhs.yz - self.yz * rhs.xy,
            xy: self.yz * rhs.xz - self.xz * rhs.xy,
        }
    }
}