use Circle;
use Polygon;
use Response;
use PolyBox;
use std::ops::{Add,Sub};
use std::f64;
use ResponseLite;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Vector2d
{
    pub x: f64,
    pub y: f64
}

impl Vector2d
{
    pub fn in_circle(&self, circle: Circle) -> bool
    {
        let difference_v = self - &circle.pos;
        let radius_sq = circle.radius * circle.radius;
        let distance_sq = difference_v.length2();
        return distance_sq <= radius_sq;
    }

    pub fn in_polygon(self, poly: Polygon) -> Option<Response>
    {
        let unit_square = PolyBox { position: self, width: 1.0, height: 1.0 }.to_polygon();
        // return test_polygon_polygon(&unit_square, &poly);
        unit_square.test(&poly)
    }

    pub fn perp(&self) -> Vector2d
    {
        let perp = Vector2d {
            x: self.y,
            y: -self.x
        };
        // ::web_bridge::log(&format!("Perpendicular {:?} {:?}", self, perp));
        perp
    }

    pub fn rotate(self, angle: f64) -> Vector2d
    {
        let new_x = self.x * angle.cos() - self.y * angle.sin();
        let new_y = self.x * angle.sin() + self.y * angle.cos();
        return Vector2d { x: new_x, y: new_y };
    }

    pub fn reverse(self) -> Vector2d
    {
        return Vector2d { x: -self.x, y: -self.y };
    }
    
    pub fn normalise(self) -> Vector2d
    {
        let length = self.length();
        if length > 0f64
        {
            return Vector2d { x: self.x / length, y: self.y / length };
        }
        else
        {
            return self;
        }
    }
    
    pub fn scale(&self, scale: f64) -> Vector2d
    {
        Vector2d { x: self.x * scale, y: self.y * scale }
    }
    
    pub fn scale_xy(self, scale_x: f64, scale_y: f64) -> Vector2d
    {
        Vector2d { x: self.x * scale_x, y: self.y * scale_y }
    }
    
    pub fn project(&self, other: &Vector2d) -> Vector2d
    {
        let amount = self.dot(&other) / other.length2();
        Vector2d { x: amount * other.x, y: amount * other.y }
    }
    
    pub fn project_n(&self, other: &Vector2d) -> Vector2d
    {
        let amount = self.dot(&other);
        Vector2d { x: amount * other.x, y: amount * other.y }
    }
    
    pub fn reflect(self, axis: Vector2d) -> Vector2d
    {
        let projection = self
           .project(&axis)
           .scale(2f64);
        Vector2d { x: projection.x - self.x, y: projection.y - self.y }
    }
    
    pub fn reflect_n(self, axis: Vector2d) -> Vector2d
    {
        let projection = self
           .project_n(&axis)
           .scale(2f64);
        Vector2d { x: projection.x - self.x, y: projection.y - self.y }
    }

    pub fn dot(&self, other: &Vector2d) -> f64
    {
        self.x * other.x + self.y * other.y
    }

    pub fn length2(&self) -> f64
    {
        self.dot(&self)
    }

    pub fn length(&self) -> f64
    {
        let dot = self.dot(&self);
        let len = dot.sqrt();
        // ::web_bridge::log(&format!("{:?} {}", self, len));
        len
    }
}

impl Add for Vector2d {
     type Output = Self;
     
     fn add(self, rhs: Self) -> Self
     {
         let x = self.x + rhs.x;
         let y = self.y + rhs.y;
         Vector2d { x, y }
     }
}

impl Sub for Vector2d {
     type Output = Self;
     
     fn sub(self, rhs: Self) -> Self
     {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Vector2d { x, y }
     }
}

impl<'a, 'b> Sub<&'b Vector2d> for &'a Vector2d {
    type Output = Vector2d;

    fn sub(self, other: &'b Vector2d) -> Vector2d {
        Vector2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<'a, 'b> Add<&'b Vector2d> for &'a Vector2d {
    type Output = Vector2d;

    fn add(self, other: &'b Vector2d) -> Vector2d {
        Vector2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn flatten_points_on(
    points: &Vec<Vector2d>,
    normal: &Vector2d) -> (f64, f64)
{
    let mut min = f64::MAX;
    let mut max = f64::MIN;
    
    for point in points
    {
        let dot = point.dot(&normal);
        if dot < min
        {
            min = dot;
        }
        if dot > max
        {
            max = dot;
        }
    }
    (min, max)
}

pub fn is_separating_axis(
    a_pos: &Vector2d,
    b_pos: &Vector2d,
    a_points: &Vec<Vector2d>,
    b_points: &Vec<Vector2d>,
    axis: &Vector2d
) -> Option<ResponseLite>
{
    let augmented_points_a: Vec<_> = a_points.iter().map(|p| a_pos + p).collect();
    let augmented_points_b: Vec<_> = b_points.iter().map(|p| b_pos + p).collect();

    let (range_a_min, range_a_max) = flatten_points_on(&augmented_points_a, &axis);
    let (range_b_min, range_b_max) = flatten_points_on(&augmented_points_b, &axis);
    
    ::web_bridge::log(&format!("Projection {} {} {:?} {:?}", range_a_min, range_a_max, augmented_points_a, axis));
    ::web_bridge::log(&format!("Projection {} {} {:?} {:?}", range_b_min, range_b_max, augmented_points_b, axis));

    let offset_v = b_pos - a_pos;
    let projected_offset = offset_v.dot(&axis);
    
    // let range_b_min = range_b_min + projected_offset;
    // let range_b_max = range_b_max + projected_offset;
    
    if range_a_min > range_b_max || range_b_min > range_a_max
    {
        return Option::None;
    }

    let a_starts_further_left_than_b = range_a_min < range_b_min;
    let (a_in_b, b_in_a, overlap) = match a_starts_further_left_than_b
    {
        true => {
            if range_a_max < range_b_max
            {
                (false, false, range_a_max - range_b_min)
            }
            else
            {
                let option1 = range_a_max - range_b_min;
                let option2 = range_b_max - range_a_min;
                let shortest_way_out = if option1 < option2 { option1 } else { -option2 };
                (false, true, shortest_way_out)
            }
        }
        false => {
            if range_a_max > range_b_max
            {
                (false, false, range_a_min - range_b_max)
            }
            else
            {
                let option1 = range_a_max - range_b_min;
                let option2 = range_b_max - range_a_min;
                let shortest_way_out = match option1 < option2 { true => option1, _ => -option2 };
                (true, false, shortest_way_out)
            }
        }
    };

    let abs_overlap = overlap.abs();
    // if (abs_overlap < response['overlap'])
    // {
    //     response['overlap'] = abs_overlap;
    //     response['overlapN'].copy(axis);
    //     if (overlap < 0)
    //     {
    //         response['overlapN'].reverse();
    //     }
    // }
    return Option::from(ResponseLite {
        a_in_b,
        b_in_a,
        overlap: abs_overlap,
        axis: *axis//: Vector2d { x: axis.x, y: axis.y }
    });
}
