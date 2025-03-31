use std::collections::HashMap;
use std::collections::HashSet;

use polyomino::point::Point;
use polyomino::point::SimplePoint;
use polyomino::polyomino::Polyomino;

use crate::colorableboard::ColorableBoard;

#[allow(dead_code)]
pub enum Orientation {
    Normal,
    OneEighty,
    FlipHorizontally,
    FlipOneEighty
}

pub fn overlay(colored_board: &ColorableBoard, blank_board: &mut ColorableBoard, orientation: Orientation) {
    match orientation {
        Orientation::Normal => {
            for x in 0..colored_board.width {
                for y in 0..colored_board.height {
                    if let Some(color) = colored_board.get_color(x, y) {
                        blank_board.set_color(x, y, color);
                    }
                }
            }
        }
        Orientation::OneEighty => {
            for x in 0..colored_board.width {
                for y in 0..colored_board.height {
                    if let Some(color) = colored_board.get_color(x, y) {
                        blank_board.set_color(colored_board.width-x-1, colored_board.height-y-1, color);
                    }
                }
            }
        }
        Orientation::FlipHorizontally => {
            for x in 0..colored_board.width {
                for y in 0..colored_board.height {
                    if let Some(color) = colored_board.get_color(x, y) {
                        blank_board.set_color(colored_board.width-x-1, y, color);
                    }
                }
            }
        }
        Orientation::FlipOneEighty => {
            for x in 0..colored_board.width {
                for y in 0..colored_board.height {
                    if let Some(color) = colored_board.get_color(x, y) {
                        blank_board.set_color(x, colored_board.height-y-1, color);
                    }
                }
            }
        }
    }
}

// Has a polyomino on the board that is colored just one color (these
// are boring and we should ignore boards that have them)
pub fn has_single_color_polyomino(board: &ColorableBoard) -> bool {
    for poly in &board.polyominoes {
        if poly.iter().all(|pt| pt.get_color() == poly[0].get_color()) {
            return true;
        }
    }

    false
}


pub fn has_all_polyomino_patterns<T: Polyomino>(board: &ColorableBoard, all_polyominoes: &Vec<Vec<T>>) -> bool {
    all_polyominoes.iter().all(|p_rotations| p_rotations.iter().any(|p| has_single_color_polyomino_pattern(board, p)))
}

// Attempts to find a set of points in the board in the pattern of the polyomino that
// are all the same color
pub fn has_single_color_polyomino_pattern<T: Polyomino>(board: &ColorableBoard, polyomino: &T) -> bool {
    for x in 0..board.get_width() {
        for y in 0..board.get_height() {
            if has_single_color_polyomino_pattern_at(board, polyomino, x, y) {
                return true;
            }
        }
    }

    false
}

// Check for two things
// * At this position, do all the spaces in the pattern of the given polyomino have the same color
// * Do all squares adjacent to this polyomino have a different color
//
// Examples:
//
// 00000
// 00100  Matches the X polyomino
// 01110
// 00100
// 00000
//
// 00100
// 00100  Does not, because of the topmost '1'
// 01110
// 00100
// 00000
//
// 00010
// 00100  Matches. Diagonally adjacent is fine
// 01110
// 00100
// 00000

fn has_single_color_polyomino_pattern_at<T: Polyomino>(board: &ColorableBoard, polyomino: &T, x:i16, y:i16) -> bool {
    let polyomino_points: Vec<SimplePoint> = polyomino.iter().map(|pt| SimplePoint::new(x+pt.x(), y+pt.y())).collect();

    let adjacent_points = build_adjacent_points(&polyomino_points);

    // make sure that all colors on this polyomino pattern are the same
    // and all colors NSEW of the pattern are different from the
    // polyomino pattern color (IOW, an isolated region)
    polyomino_points.iter().all(|pt| board.get_color(pt.x(), pt.y()).is_some() &&
                                board.get_color(pt.x(), pt.y()) == board.get_color(x+polyomino.get_nth(0).unwrap().x(),
                                                                                   y+polyomino.get_nth(0).unwrap().y()))
        && adjacent_points.iter().all(|pt| board.get_color(pt.x(), pt.y()).is_none() ||
                                      board.get_color(pt.x(), pt.y()) != board.get_color(x+polyomino.get_nth(0).unwrap().x(),
                                                                                         y+polyomino.get_nth(0).unwrap().y()))
        
}

// Return a set of all points NSEW adjacent to the specified vector of
// points Note that this can include bogus points with negative x and
// y (or larger than the dimentions. These points are only tested for
// color and points outside the board return None for get_color()
fn build_adjacent_points(points: &Vec<SimplePoint>) -> HashSet<SimplePoint> {
    let mut adjacent = HashSet::new();

    points.iter().for_each(|pt| { adjacent.insert(SimplePoint::new(pt.x()+1, pt.y()));
                                  adjacent.insert(SimplePoint::new(pt.x()-1, pt.y()));
                                  adjacent.insert(SimplePoint::new(pt.x(), pt.y()+1));
                                  adjacent.insert(SimplePoint::new(pt.x(), pt.y()-1));
    });

    for point in points {
        adjacent.remove(point);
    }

    adjacent
}


pub fn color_count(solution: &ColorableBoard) -> Vec<i32> {
    let mut used_colors = HashMap::new();
    
    for x in 0..solution.width {
        for y in 0..solution.height {
            let color = solution.get_color(x, y).unwrap();
            match used_colors.get(&color) {
                None => { used_colors.insert(color, 1); }
                Some(count) => { used_colors.insert(color, count+1); }
            }
        }
    }

    used_colors.values().cloned().collect()
}

#[cfg(test)]
mod tests {
    use polyomino::board::Board;
    use polyomino::point::Point;
    use polyomino::point::SimplePoint;
    use polyomino::polyomino::Polyomino;
    use polyomino::polyomino::SimplePolyomino;
    use polyomino::solver::Solver;
    use polyomino::utils as poly_utils;
    use polyomino::utils::Restrictions;
    use polyomino::utils::PredefinedPolyominoes;
    use crate::boardcolorer::color_board;
    use crate::colorableboard::ColorableBoard;
    use crate::colorableboard::IndexedBoardState;
    use crate::colorpoint::ColorPoint;
    use crate::colorpolyomino::ColorPolyomino;
    
    use crate::utils::*;

    //  +-+-+-+-+-+
    //  |0  | | | |
    //  +   + + + +
    //  |0  | |0| |
    //  + +-+ + + +
    //  |0|  0|0|0|
    //  +-+ +-+ + +
    //  |0| |  0| |
    //  + +-+-+-+ +
    //  |0      | |
    //  +-+-+-+-+-+
    
    fn build_board() -> ColorableBoard {
        let mut board = ColorableBoard {
            height: 5,
            width: 5,
            board: Vec::new(),
            polyominoes: Vec::new()
        };

        // P
        let mut v = Vec::new();
        v.push(ColorPoint::build_point(0, 0, '0'));
        v.push(ColorPoint::build_point(0, 1, '0'));
        v.push(ColorPoint::build_point(0, 2, '0'));
        v.push(ColorPoint::build_point(1, 0, '1'));
        v.push(ColorPoint::build_point(1, 1, '1'));

        board.polyominoes.push(ColorPolyomino::new(v));

        // L at bottom
        v = Vec::new();
        v.push(ColorPoint::build_point(0, 0, '0'));
        v.push(ColorPoint::build_point(0, 1, '0'));
        v.push(ColorPoint::build_point(1, 1, '1'));
        v.push(ColorPoint::build_point(2, 1, '1'));
        v.push(ColorPoint::build_point(3, 1, '1'));
        
        board.polyominoes.push(ColorPolyomino::new(v));

        // I on side
        v = Vec::new();
        v.push(ColorPoint::build_point(0, 0, '1'));
        v.push(ColorPoint::build_point(0, 1, '1'));
        v.push(ColorPoint::build_point(0, 2, '0'));
        v.push(ColorPoint::build_point(0, 3, '1'));
        v.push(ColorPoint::build_point(0, 4, '1'));
        
        board.polyominoes.push(ColorPolyomino::new(v));

        // Backwards L
        v = Vec::new();
        v.push(ColorPoint::build_point(0, 3, '1'));
        v.push(ColorPoint::build_point(1, 0, '1'));
        v.push(ColorPoint::build_point(1, 1, '0'));
        v.push(ColorPoint::build_point(1, 2, '0'));
        v.push(ColorPoint::build_point(1, 3, '0'));
        
        board.polyominoes.push(ColorPolyomino::new(v));

        // Zig-zag
        v = Vec::new();
        v.push(ColorPoint::build_point(0, 2, '1'));
        v.push(ColorPoint::build_point(0, 3, '1'));
        v.push(ColorPoint::build_point(1, 0, '1'));
        v.push(ColorPoint::build_point(1, 1, '1'));
        v.push(ColorPoint::build_point(1, 2, '0'));
        board.polyominoes.push(ColorPolyomino::new(v));

        board.board.push(IndexedBoardState::Full(0, 0, 0, 0));
        board.board.push(IndexedBoardState::Full(0, 3, 0, 0));
        board.board.push(IndexedBoardState::Full(4, 2, 2, 0));
        board.board.push(IndexedBoardState::Full(3, 1, 3, 0));
        board.board.push(IndexedBoardState::Full(2, 0, 4, 0));
        
        board.board.push(IndexedBoardState::Full(0, 2, 0, 0));
        board.board.push(IndexedBoardState::Full(0, 4, 0, 0));
        board.board.push(IndexedBoardState::Full(4, 3, 2, 0));
        board.board.push(IndexedBoardState::Full(3, 2, 3, 0));
        board.board.push(IndexedBoardState::Full(2, 1, 4, 0));
        
        board.board.push(IndexedBoardState::Full(0, 2, 0, 0));
        board.board.push(IndexedBoardState::Full(4, 0, 2, 0));
        board.board.push(IndexedBoardState::Full(4, 4, 2, 0));
        board.board.push(IndexedBoardState::Full(3, 3, 3, 0));
        board.board.push(IndexedBoardState::Full(2, 2, 4, 0));
        
        board.board.push(IndexedBoardState::Full(1, 0, 0, 3));
        board.board.push(IndexedBoardState::Full(4, 1, 2, 0));
        board.board.push(IndexedBoardState::Full(3, 0, 2, 0));
        board.board.push(IndexedBoardState::Full(3, 4, 3, 0));
        board.board.push(IndexedBoardState::Full(2, 3, 4, 0));
        
        board.board.push(IndexedBoardState::Full(1, 1, 0, 3));
        board.board.push(IndexedBoardState::Full(1, 2, 0, 3));
        board.board.push(IndexedBoardState::Full(1, 3, 0, 3));
        board.board.push(IndexedBoardState::Full(1, 4, 0, 3));
        board.board.push(IndexedBoardState::Full(2, 4, 4, 0));

        board
    }

    fn build_x() -> SimplePolyomino<SimplePoint> {
        let mut v = Vec::new();
        v.push(SimplePoint::new(1, 0));
        v.push(SimplePoint::new(0, 1));
        v.push(SimplePoint::new(1, 1));
        v.push(SimplePoint::new(2, 1));
        v.push(SimplePoint::new(1, 2));

        SimplePolyomino::new(v)
    }

    fn build_flat_l() -> SimplePolyomino<SimplePoint> {
        let mut v = Vec::new();
        v.push(SimplePoint::new(0, 0));
        v.push(SimplePoint::new(1, 0));
        v.push(SimplePoint::new(2, 0));
        v.push(SimplePoint::new(3, 0));
        v.push(SimplePoint::new(4, 0));

        SimplePolyomino::new(v)
    }
    
    fn build_l() -> SimplePolyomino<SimplePoint> {
        let mut v = Vec::new();
        v.push(SimplePoint::new(0, 0));
        v.push(SimplePoint::new(0, 1));
        v.push(SimplePoint::new(0, 2));
        v.push(SimplePoint::new(0, 3));
        v.push(SimplePoint::new(0, 4));

        SimplePolyomino::new(v)
    }

    #[test]
    fn has_x() {
        let b = build_board();

        let piece = build_x();

        assert!(has_single_color_polyomino_pattern(&b, &piece));
    }
    
    #[test]
    fn has_l() {
        let b = build_board();

        let piece = build_l();

        assert!(has_single_color_polyomino_pattern(&b, &piece));
    }

    #[test]
    fn does_not_have_flat_l() {
        let b = build_board();

        let piece = build_flat_l();

        assert!(!has_single_color_polyomino_pattern(&b, &piece));
    }

    #[test]
    fn space_too_big() {
        let mut b = build_board();

        // Add another '0' on top of the +
        b.set_color(3, 0, '0');
        
        let piece = build_x();

        assert!(!has_single_color_polyomino_pattern(&b, &piece));
    }

    #[test]
    fn has_all_variations_of_patterns() {
        if let Ok(mut polyominoes) = poly_utils::get_polyominoes::<ColorPolyomino>(PredefinedPolyominoes::Pentominoes) {
            polyominoes.iter_mut().enumerate().for_each(|(id, p)| p.set_id(id));
            
            let all_polyominoes = poly_utils::build_variations(&polyominoes, Restrictions::RectangularSymmetry);
            
            let mut b = Board::new(15, 4);
            
            let mut solver = Solver::new(&mut b, &all_polyominoes);
            
            let solutions:Vec<ColorableBoard> = solver.solve().iter().map(|s| ColorableBoard::new(s)).collect();

            // 10 and 58 were chosen at random
            let mut soln10 = solutions[10].clone();
            let mut soln58 = solutions[58].clone();
            color_board(&mut soln10);
            overlay(&soln10, &mut soln58, Orientation::Normal);

            assert!(has_single_color_polyomino(&soln10));
            
            assert!(!has_single_color_polyomino(&soln58));

            // This one has the right color pattern because all the polyominoes are mono-colored
            println!("Solution #10\n{}", soln10);
            assert!(has_all_polyomino_patterns(&soln10, &all_polyominoes));
            
            // This one has the right color pattern because even though the polyominoes are not
            // (necessarily) mono-colored, each shape is represented
            println!("Solution #58\n{}", soln58);
            assert!(has_all_polyomino_patterns(&soln58, &all_polyominoes));

            // Change the color of one point (any one will do) and we should no longer have all the
            // patterns represented
            let mut point = soln58.polyominoes[2].get_nth_mut(0).unwrap();

            point.set_color(
                if point.get_color() == '3' {
                    '2'
                } else {
                    '3'
                });
            
            println!("Modified solution #58\n{}", soln58);
            assert!(!has_all_polyomino_patterns(&soln58, &all_polyominoes));
        }
    }
}
