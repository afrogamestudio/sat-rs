use Vector2d;

#[derive(Debug)]
pub struct ResponseLite
{
    pub a_in_b: bool,
    pub b_in_a: bool,
    pub overlap: f64,
    pub axis: Vector2d
}
