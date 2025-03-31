use std::fmt;
use std::ops::{Index, IndexMut};
use std::slice::Iter;

use polyomino::polyomino::Polyomino;

use crate::colorpoint::ColorPoint;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct ColorPolyomino {
    id: usize,
    points: Vec<ColorPoint>,
}

impl Polyomino for ColorPolyomino {
    type Pt = ColorPoint;

    fn new(mut points: Vec<ColorPoint>) -> ColorPolyomino {
        points.sort();
        points.dedup();
        ColorPolyomino { id: 0,
                         points }
    }

    fn iter(&self) -> Iter<Self::Pt> {
        self.points.iter()
    }

    fn get_nth(&self, nth: usize) -> Option<&Self::Pt> {
        self.points.get(nth)
    }

    fn set_points(&mut self, mut points: Vec<Self::Pt>) {
        points.sort();
        points.dedup();
        self.points = points;
    }
}

#[allow(dead_code)]
impl ColorPolyomino {
    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
    
    pub fn set_color(&mut self, color: char) {
        for pt in &mut self.points {
            pt.set_color(color);
        }
    }
    
    pub fn get_nth_mut(&mut self, nth: usize) -> Option<&mut <ColorPolyomino as Polyomino>::Pt> {
        self.points.get_mut(nth)
    }

}

impl fmt::Display for ColorPolyomino {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.show(f)
    }
}

impl Index<usize> for ColorPolyomino {
    type Output = ColorPoint;
    fn index(&self, i: usize) -> &ColorPoint {
        &self.points[i]
    }
}

impl IndexMut<usize> for ColorPolyomino {
    fn index_mut(&mut self, i: usize) -> &mut ColorPoint {
        &mut self.points[i]
    }
}

#[cfg(test)]
mod tests {
    use polyomino::point::Point;
    use polyomino::polyomino::Polyomino;
    use polyomino::utils;
    use polyomino::utils::Restrictions;
    use crate::colorpoint::ColorPoint;
    use crate::colorpolyomino::ColorPolyomino;

    fn build_f_pentomino() -> ColorPolyomino {
        let mut v = Vec::new();

        v.push(ColorPoint::build_point(0, 1, 'a'));
        v.push(ColorPoint::build_point(1, 1, 'b'));
        v.push(ColorPoint::build_point(1, 0, 'c'));
        v.push(ColorPoint::build_point(2, 2, 'd'));
        v.push(ColorPoint::build_point(1, 2, 'e'));

        ColorPolyomino::new(v)
    }

    #[test]
    fn test_f() {
        let f = build_f_pentomino();

        assert_eq!(f.get_nth(0).unwrap().get_color(), 'a');
        assert_eq!(f.get_nth(0).unwrap().x(), 0);
        assert_eq!(f.get_nth(0).unwrap().y(), 1);

        let rot_f = f.rotate();
        
        assert_eq!(rot_f.get_nth(0).unwrap().get_color(), 'e');
        assert_eq!(rot_f.get_nth(0).unwrap().x(), 0);
        assert_eq!(rot_f.get_nth(0).unwrap().y(), 1);
    }

    fn build_l_pentomino() -> ColorPolyomino {
        let mut v = Vec::new();

        v.push(ColorPoint::build_point(0, 0, '0'));
        v.push(ColorPoint::build_point(0, 1, '1'));
        v.push(ColorPoint::build_point(0, 2, '1'));
        v.push(ColorPoint::build_point(0, 3, '1'));
        v.push(ColorPoint::build_point(0, 4, '1'));

        ColorPolyomino::new(v)
    }

    fn build_symmetric_l_pentomino() -> ColorPolyomino {
        let mut v = Vec::new();

        v.push(ColorPoint::build_point(0, 0, '0'));
        v.push(ColorPoint::build_point(0, 1, '1'));
        v.push(ColorPoint::build_point(0, 2, '1'));
        v.push(ColorPoint::build_point(0, 3, '1'));
        v.push(ColorPoint::build_point(0, 4, '0'));

        ColorPolyomino::new(v)
    }

    #[test]
    fn l_l() {
        let l = build_l_pentomino();
        assert!(l == l);
    }

    #[test]
    fn l_is_not_l_double_rot() {
        let l = build_l_pentomino();
        assert!(l != l.rotate().rotate());
    }
    
    #[test]
    fn l_is_l_360() {
        let l = build_l_pentomino();
        assert!(l == l.rotate().rotate().rotate().rotate());
    }

    #[test]
    fn count_l_variations() {
        let polys = vec![build_l_pentomino(), build_symmetric_l_pentomino()];

        let all_polys = utils::build_variations(&polys, Restrictions::RectangularSymmetry);

        assert!(all_polys[0].len() == 4);
        assert!(all_polys[1].len() == 2);
    }
}













