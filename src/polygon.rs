use Vector2d;
use response::Response;
use ResponseLite;
use vector2d::is_separating_axis;

//#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Polygon
{
    pub position: Vector2d,
    // angle: f64,
    // offset: Vector2d,
    // calc_points: Vec<Vector2d>,
    // edges: Vec<Vector2d>,
    // normals: Vec<Vector2d>,
    pub points: Vec<Vector2d>
}

impl Polygon {
    pub fn set_points(self, points: Vec<Vector2d>) -> Polygon
    {
        Polygon {
            points,
            ..self
        }
    }

    pub fn test(&self, b: &Polygon) -> Option<Response>
    {
        let normals_a = self.calculate_normals();
        let normals_b = b.calculate_normals();

        // ::web_bridge::log(&format!("{:?} {:?}", normals_a, normals_b));

        let sats_a : Vec<Option<ResponseLite>> = normals_a
            .iter()
            .map(|normal| is_separating_axis(&self.position, &b.position, &self.points, &b.points, &normal))
            .collect();

        let sats_b : Vec<Option<ResponseLite>> = normals_b
            .iter()
            .map(|normal| is_separating_axis(&self.position, &b.position, &self.points, &b.points, &normal))
            .collect();

        let sats: Vec<_> = sats_a.into_iter().chain(sats_b.into_iter()).collect();

        let is_not_collision = sats.iter().any(|x| x.is_none());

        if is_not_collision
        {
            return Option::None;
        }

        let sats2 : Vec<ResponseLite> = sats.into_iter().map(|x| x.unwrap()).collect();
        ::web_bridge::log(&format!("SATS {:?}", sats2));
        
        let a_in_b = sats2.iter().all(|collision| collision.a_in_b );
        let b_in_a = sats2.iter().all(|collision| collision.b_in_a );
        let min_collision = sats2.iter()
            .min_by_key(|collision| (collision.overlap * 1000.0) as i32)
            .unwrap();
        let overlap_v = min_collision.axis.scale(min_collision.overlap);
        Option::from(Response {
            a: self.clone(),
            b: b.clone(),
            a_in_b,
            b_in_a,
            overlap: min_collision.overlap,
            overlap_n: min_collision.axis,//.scale(1.0),
            overlap_v
        })
    }

    pub fn calculate_normals(&self) -> Vec<Vector2d>
    {
        let len = self.points.len();
        self
            .points
            .iter()
            .enumerate()
            .map(|(index, p)| (p, if index < len - 1 { self.points[index + 1] } else { self.points[0] }))
            .map(|(p1, p2)| &p2 - p1)
            .map(|p| p.perp().normalise())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Vector2d;
    use super::Polygon;

    #[test]
    fn returns_no_collisions_if_polygons_do_not_overlap() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn returns_collision_if_polygons_overlap() {
        
    }

    #[test]
    fn should_scale_by_zero_properly() {
        // var V = SAT.Vector;
        // var v1 = new V(5, 5);
        // v1.scale(10, 10);
        // assert(v1.x === 50);
        // assert(v1.y === 50);

        // v1.scale(0, 1);
        // assert(v1.x === 0);
        // assert(v1.y === 50);

        // v1.scale(1, 0);
        // assert(v1.x === 0);
        // assert(v1.y === 0);
    }

    #[test]
    fn should_calculate_the_correct_value_for_a_square() {
        // var V = SAT.Vector;
        // var P = SAT.Polygon;

        // // A square
        // var polygon = new P(new V(0,0), [
        // new V(0,0), new V(40,0), new V(40,40), new V(0,40)
        // ]);
        // var c = polygon.getCentroid();
        // assert( c.x === 20 );
        // assert( c.y === 20 );
    }

    #[test]
    fn should_calculate_the_correct_value_for_a_triangle() {
        // var V = SAT.Vector;
        // var P = SAT.Polygon;

        // // A triangle
        // var polygon = new P(new V(0,0), [
        // new V(0,0), new V(100,0), new V(50,99)
        // ]);
        // var c = polygon.getCentroid();
        // assert( c.x === 50 );
        // assert( c.y === 33 );
    }

    #[test]
    fn test_circle_circle() {
        // var V = SAT.Vector;
        // var C = SAT.Circle;

        // var circle1 = new C(new V(0,0), 20);
        // var circle2 = new C(new V(30,0), 20);
        // var response = new SAT.Response();
        // var collided = SAT.testCircleCircle(circle1, circle2, response);

        // assert( collided );
        // assert( response.overlap == 10 );
        // assert( response.overlapV.x == 10 && response.overlapV.y === 0);
    }

    #[test]
    fn test_polygon_circle() {

        // var V = SAT.Vector;
        // var C = SAT.Circle;
        // var P = SAT.Polygon;

        // var circle = new C(new V(50,50), 20);
        // // A square
        // var polygon = new P(new V(0,0), [
        // new V(0,0), new V(40,0), new V(40,40), new V(0,40)
        // ]);
        // var response = new SAT.Response();
        // var collided = SAT.testPolygonCircle(polygon, circle, response);

        // assert(collided);
        // assert(response.overlap.toFixed(2) == "5.86");
        // assert(
        // response.overlapV.x.toFixed(2) == "4.14" &&
        // response.overlapV.y.toFixed(2) == "4.14"
        // );
    }

    #[test]
    fn test_polygon_polygon_a() {
        // var V = SAT.Vector;
        // var P = SAT.Polygon;

        // // A square
        // var polygon1 = new P(new V(0,0), [
        // new V(0,0), new V(40,0), new V(40,40), new V(0,40)
        // ]);
        // // A triangle
        // var polygon2 = new P(new V(30,0), [
        // new V(0,0), new V(30, 0), new V(0, 30)
        // ]);
        // var response = new SAT.Response();
        // var collided = SAT.testPolygonPolygon(polygon1, polygon2, response);

        // assert( collided );
        // assert( response.overlap == 10 );
        // assert( response.overlapV.x == 10 && response.overlapV.y === 0);

        let polygon1 = Polygon {
            position: Vector2d {
                x: 0.0,
                y: 0.0
            },
            points: vec![
                Vector2d { x: 0.0, y: 0.0 },
                Vector2d { x: 40.0, y: 0.0 }, 
                Vector2d { x: 40.0, y: 40.0 },
                Vector2d { x: 0.0, y: 40.0 }
            ]
        };

        let polygon2 = Polygon {
            position: Vector2d {
                x: 30.0, y: 0.0
            },
            points: vec![
                Vector2d { x: 0.0, y: 0.0 },
                Vector2d { x: 30.0, y: 0.0 }, 
                Vector2d { x: 0.0, y: 30.0 }
            ]
        };

        let response = polygon1.test(&polygon2);

        assert!(response.is_some());
        let collision = response.unwrap();
        assert_eq!(10.0, collision.overlap);
        assert_eq!(10.0, collision.overlap_v.x);
        assert_eq!(0.0, collision.overlap_v.y);
        // assert( response.overlapV.x == 10 && response.overlapV.y === 0);
    }

    #[test]
    fn test_polygon_polygon_b() {
        // var V = SAT.Vector;
        // var B = SAT.PolyBox;

        // var box1 = new B(new V(0,0), 20, 20).toPolygon();
        // var box2 = new B(new V(100,100), 20, 20).toPolygon();
        // var collided = SAT.testPolygonPolygon(box1, box2);
    }

    #[test]
    fn point_in_circle() {
        // var V = SAT.Vector;
        // var C = SAT.Circle;

        // var circle = new C(new V(100,100), 20);

        // assert(!SAT.pointInCircle(new V(0,0), circle)); // false
        // assert(SAT.pointInCircle(new V(110,110), circle)); // true
    }

    #[test]
    fn point_in_polygon() {
        // var V = SAT.Vector;
        // var C = SAT.Circle;
        // var P = SAT.Polygon;

        // var triangle = new P(new V(30,0), [
        // new V(0,0), new V(30, 0), new V(0, 30)
        // ]);
        // assert(!SAT.pointInPolygon(new V(0,0), triangle)); // false
        // assert(SAT.pointInPolygon(new V(35, 5), triangle)); // true
    }

    #[test]
    fn point_in_polygon_b() {
        // var V = SAT.Vector;
        // var C = SAT.Circle;
        // var P = SAT.Polygon;

        // var v1 = new V(1, 1.1);
        // var p1 = new P(new V(0,0),[new V(2,1), new V(2,2), new V(1,3), new V(0,2),new V(0,1),new V(1,0)]);
        // assert(SAT.pointInPolygon(v1, p1));
    }
}

