
use std::cmp::Ordering;
use std::fmt;
use std::hash::Hash;

use colored::Colorize;
use colored::Color;

use polyomino::point::Point;


#[derive(Debug, Clone, Copy)]
pub struct ColorPoint {
    x: i16,
    y: i16,
    color: char
}

impl Point for ColorPoint {
    fn new(x: i16, y: i16) -> ColorPoint {
        ColorPoint { x, y, color: '0' }
    }

    fn x(&self) -> i16 {
        self.x
    }

    fn set_x(&mut self, x: i16) {
        self.x = x;
    }
    
    fn y(&self) -> i16 {
        self.y
    }
    
    fn set_y(&mut self, y: i16) {
        self.y = y;
    }
}

impl PartialOrd for ColorPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        std::option::Option::Some(self.cmp(other))
    }
}

impl Ord for ColorPoint {
    fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
        if self.x() < other.x() {
            return Ordering::Less;
        }
        
        if self.x() > other.x() {
            return Ordering::Greater;
        }

        if self.y() != other.y() {
            self.y().cmp(&other.y())
        } else {
            self.get_color().cmp(&other.get_color())
        }
    }
}

impl PartialEq for ColorPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.get_color() == other.get_color()
    }
}

impl Eq for ColorPoint {
}

impl Hash for ColorPoint {
    fn hash<H :std::hash::Hasher>(&self, state: &mut H) {
        self.x().hash(state);
        self.y().hash(state);
        self.get_color().hash(state);
    }
}

impl fmt::Display for ColorPoint {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "\u{25A0}".color(make_color(self.color)))
    }
}


pub fn make_color(color: char) -> Color {
    match color {
        '0' => Color::Red,
        '1' => Color::Blue,
        '2' => Color::Yellow,
        '3' => Color::Green,
        _ => Color::Black
    }
}

#[allow(dead_code)]
impl ColorPoint {
    pub fn build_point(x:i16, y:i16, color: char) -> ColorPoint {
        ColorPoint { x, y, color }
    }
    
    pub fn get_color(&self) -> char {
        self.color
    }

    pub fn set_color(&mut self, color: char) {
        self.color = color
    }
}

#[cfg(test)]
mod tests {
    use crate::colorpoint::ColorPoint;

    #[test]
    fn pt_eq_self() {
        let p = ColorPoint::build_point(0, 0, '0');

        assert!(p == p);
    }
    
    #[test]
    fn pt_eq_same() {
        let p1 = ColorPoint::build_point(0, 0, '0');
        let p2 = ColorPoint::build_point(0, 0, '0');

        assert!(p1 == p2);
    }
    
    #[test]
    fn pt_color_mismatch() {
        let p1 = ColorPoint::build_point(0, 0, '0');
        let p2 = ColorPoint::build_point(0, 0, '1');

        assert!(p1 != p2);
    }
}

