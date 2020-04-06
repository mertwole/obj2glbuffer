pub struct Vec3{
    pub x : f32,
    pub y : f32,
    pub z : f32
}

impl Vec3{
    pub fn new(x : f32, y : f32, z : f32) -> Vec3{
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3{
        Vec3 { x : 0.0, y : 0.0, z : 0.0 }
    }

    pub fn clone(&self) -> Vec3{
        Vec3 { x : self.x, y : self.y, z : self.z }
    }

    pub fn normalized(&self) -> Vec3 {
        let sqr_len = self.x * self.x + self.y * self.y + self.z * self.z;
        let len = sqr_len.sqrt();
        Vec3 { x : self.x / len, y : self.y / len, z : self.z / len }
    }
}

impl std::ops::Div<f32> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Vec3 {
        Vec3 { x : self.x / rhs, y : self.y / rhs, z : self.z / rhs }
    }
}