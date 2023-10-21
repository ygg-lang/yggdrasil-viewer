use num::Float;
use std::cmp::{max, min};

pub type Coord = f32;

#[derive(PartialEq, Debug)]
pub struct Point {
    pub x: Coord,
    pub y: Coord,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Orientation {
    ClockWise,
    CounterClockWise,
    Colinear,
}

impl Point {
    pub fn orientation(p: &Point, q: &Point, r: &Point) -> Orientation {
        let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);

        if val.abs() < 1e-7 {
            Orientation::Colinear
        }
        else if val > 0. {
            Orientation::ClockWise
        }
        else {
            Orientation::CounterClockWise
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn orient() {
        let a = Point { x: 0., y: 0. };
        let b = Point { x: 1., y: 0. };
        let c = Point { x: 0., y: 1. };
        assert_eq!(Point::orientation(&a, &b, &c), Orientation::CounterClockWise);
        let a = Point { x: 0., y: 0. };
        let b = Point { x: 0., y: 1. };
        let c = Point { x: 1., y: 0. };
        assert_eq!(Point::orientation(&a, &b, &c), Orientation::ClockWise);
        let a = Point { x: 0., y: 0. };
        let b = Point { x: 1., y: 1. };
        let c = Point { x: 4., y: 4. };
        assert_eq!(Point::orientation(&a, &b, &c), Orientation::Colinear);
    }
}
