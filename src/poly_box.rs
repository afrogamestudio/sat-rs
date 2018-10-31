use super::Vector2d;
use super::Polygon;

#[derive(Serialize, Deserialize)]
pub struct PolyBox
{
    pub position: Vector2d,
    pub width: f64,
    pub height: f64
}

impl PolyBox {
     pub fn to_polygon(self) -> Polygon
     {
         Polygon {
             position: self.position,
             points: vec![
                 Vector2d { x: 0f64, y: 0f64 },
                 Vector2d { x: self.width, y: 0f64 },
                 Vector2d { x: self.width, y: self.height },
                 Vector2d { x: 0f64, y: self.height }
             ]
         }
     }
}