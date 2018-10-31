use super::Vector2d;
use super::PolyBox;

#[derive(Serialize, Deserialize)]
pub struct Circle
{
    pub pos: Vector2d,
    pub radius: f64
}

impl Circle {
    pub fn get_axis_aligned_bounding_box(self) -> PolyBox
    {
        let corner = self.pos - Vector2d { x: self.radius, y: self.radius };
        PolyBox { position: corner, width: self.radius * 2f64, height: self.radius * 2f64 }
    }
}
