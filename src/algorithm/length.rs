use num_traits::Float;

use types::{MultiLineString};
use traits::{PointTrait, LineStringTrait};

/// Calculation of the length

pub trait Length<T, RHS = Self> {
    /// Calculation the length of a Line
    ///
    /// ```
    /// /*
    /// use geo::{Point, LineString, Coordinate};
    /// use geo::algorithm::length::Length;
    ///
    /// let mut vec = Vec::new();
    /// vec.push(Point::new(40.02f64, 116.34));
    /// vec.push(Point::new(42.02f64, 116.34));
    /// let linestring = LineString(vec);
    ///
    /// println!("Length {}", linestring.length());
    /// */
    /// ```
    ///
    fn length(&self) -> T;
}

pub fn line_string<'a, G, T>(line_string: &'a G) -> T 
    where T: 'a + Float + ::num_traits::FromPrimitive,
          G: 'a + LineStringTrait<'a, T> + ?Sized
{
    // FIXME: don't collect
    let v = line_string.points().collect::<Vec<_>>();
    v.windows(2)
     .fold(T::zero(), |total_length, p| total_length + p[0].distance_to_point(&p[1]))
}

impl<T> Length<T> for MultiLineString<T>
    where T: Float + ::num_traits::FromPrimitive
{
    fn length(&self) -> T {
        self.0.iter().fold(T::zero(), |total, line| total + line.length())
    }
}

#[cfg(test)]
mod test {
    use types::{Coordinate, Point, LineString, MultiLineString};
    use algorithm::length::Length;
    use traits::{LineStringTrait};

    #[test]
    fn empty_linestring_test() {
        let linestring = LineString::<f64>(Vec::new());
        assert_eq!(0.0_f64, linestring.length());
    }
    #[test]
    fn linestring_one_point_test() {
        let linestring = LineString(vec![Point::new(0., 0.)]);
        assert_eq!(0.0_f64, linestring.length());
    }
    #[test]
    fn linestring_test() {
        let p = |x| Point(Coordinate { x: x, y: 1. });
        let linestring = LineString(vec![p(1.), p(7.), p(8.), p(9.), p(10.), p(11.)]);
        assert_eq!(10.0_f64, linestring.length());
    }
    #[test]
    fn multilinestring_test() {
        let p = |x, y| Point(Coordinate { x: x, y: y });
        let mline = MultiLineString(vec![LineString(vec![p(1., 0.), p(7., 0.), p(8., 0.), p(9., 0.), p(10., 0.), p(11., 0.)]),
                                         LineString(vec![p(0., 0.), p(0., 5.)])]);
        assert_eq!(15.0_f64, mline.length());
    }
}
