use super::Vector2d;
use super::Polygon;

#[derive(Serialize, Deserialize)]
pub struct Response
{
    pub a: Polygon,
    pub b: Polygon,
    pub overlap_n: Vector2d,
    pub overlap_v: Vector2d,
    pub overlap: f64,
    pub a_in_b: bool,
    pub b_in_a: bool
}
