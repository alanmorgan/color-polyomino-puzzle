use std::fmt;

use colored::Colorize;

use polyomino::board::Board;
use polyomino::board::BoardState;
use polyomino::polyomino::Polyomino;

use crate::colorpoint::ColorPoint;
use crate::colorpoint::make_color;
use crate::colorpolyomino::ColorPolyomino;

// Similar to a polyomino::board::Board, but these references
// to the underlying polyominoes are mutable so we can color them

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum IndexedBoardState {
    Void,
    Empty,
    Full(usize, usize, i16, i16),   // polyomino index, piece index, x,y position of first piece in polyomino
}

#[allow(dead_code)]
impl IndexedBoardState {
    pub fn connected_to(&self, other : IndexedBoardState) -> bool {
        match *self {
            IndexedBoardState::Void => other == IndexedBoardState::Void,
            IndexedBoardState::Empty => other == IndexedBoardState::Empty,
            IndexedBoardState::Full(pidx1, _, _, _) => if let IndexedBoardState::Full(pidx2, _, _, _) = other { pidx1 == pidx2 } else { false }
        }
    }

    pub fn get_poly_idx(&self) -> Option<usize> {
        if let IndexedBoardState::Full(idx, _, _, _) = *self {
            Some(idx)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct ColorableBoard {
    pub height: i16,
    pub width: i16,
    pub board: Vec<IndexedBoardState>,
    pub polyominoes: Vec<ColorPolyomino>
}

#[allow(dead_code)]
impl ColorableBoard {
    pub fn new(b: &Board<ColorPolyomino>) -> ColorableBoard {
        let mut mb : ColorableBoard = ColorableBoard {
            height: b.get_height(),
            width: b.get_width(),
            board: vec![IndexedBoardState::Empty; (b.get_height() * b.get_width()) as usize],
            polyominoes: Vec::new()
        };

        let mut idx:usize = 0;
        
        for y in 0..b.get_height() {
            for x in 0..b.get_width() {
                mb.board[idx] = match b.get(x, y) {
                    BoardState::Void => IndexedBoardState::Void,
                    BoardState::Empty => IndexedBoardState::Empty,
                    BoardState::Full(p, pt, px, py) => {
                        IndexedBoardState::Full(Self::find_insert(&mut mb.polyominoes, p), pt, px, py)
                    }
                };
                idx += 1
            }
        }

        mb
    }
    
    pub fn get(&self, x: i16, y: i16) -> IndexedBoardState {
        if self.on_board(x, y) {
            self.board[self.to_idx(x, y)]
        } else {
            IndexedBoardState::Void
        }
    }

    pub fn get_color(&self, x: i16, y: i16) -> Option<char> {
        if let IndexedBoardState::Full(p, pt, _x, _y) = self.get(x, y) {
            Some(self.get_point(p, pt).get_color())
        } else {
            None
        }
    }
    
    pub fn set_color(&mut self, x: i16, y: i16, color: char) {
        if let IndexedBoardState::Full(p, pt, _x, _y) = self.get(x, y) {
            self.polyominoes[p].get_nth_mut(pt).unwrap().set_color(color);
        }
    }
    
    fn to_idx(&self, x: i16, y: i16) -> usize {
        (x+ y * self.width) as usize
    }

    fn on_board(&self, x: i16, y: i16) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height
    }
    
    pub fn get_height(&self) -> i16 {
        self.height
    }

    pub fn get_width(&self) -> i16 {
        self.width
    }

    pub fn get_polyomino(&self, poly_index: usize) -> &ColorPolyomino {
        &self.polyominoes[poly_index]
    }
    
    pub fn get_polyomino_mut(&mut self, poly_index: usize) -> &mut ColorPolyomino {
        &mut self.polyominoes[poly_index]
    }

    pub fn get_point(&self, poly_index: usize, pt_index: usize) -> &ColorPoint {
        self.polyominoes[poly_index].get_nth(pt_index).unwrap()
    }
    
    fn find_insert(v: &mut Vec<ColorPolyomino>, needle: &ColorPolyomino) -> usize {
        match v.iter().position(|p| p == needle) {
            None => {
                v.push(needle.clone());
                v.len()-1
            }
            Some(idx) => {
                idx
            }
        }
    }
}

// Largely a copy of the polyomino code. Could be made generic if the board state were turned into a trait
impl fmt::Display for ColorableBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn print_top_row_border(s: &ColorableBoard, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str(if s.get(0, 0) == IndexedBoardState::Void {
                " "
            } else {
                "+"
            })?;

            for x in 0..s.width {
                let piece = s.get(x, 0);

                f.write_str(if piece == IndexedBoardState::Void {
                    if s.get(x + 1, 0) == IndexedBoardState::Void {
                        "  "
                    } else {
                        " +"
                    }
                } else {
                    "-+"
                })?;
            }

            f.write_str("\n")
        }

        fn print_row(s: &ColorableBoard, f: &mut fmt::Formatter, y: i16) -> fmt::Result {
            for x in 0..s.width {
                let piece = s.get(x, y);

                if x == 0 {
                    f.write_str(if piece == IndexedBoardState::Void { " " } else { "|" })?;
                }

                write!(f, "{}",
                       match piece {
                           IndexedBoardState::Void => " ".black(),
                           IndexedBoardState::Empty => ".".white(),
                           IndexedBoardState::Full(p_idx, pt_idx, _x, _y) => {
                               "\u{25A0}".color(make_color(s.get_point(p_idx, pt_idx).get_color()))
                           }
                       })?;
                        
                // Testing equivalence should ignore the particular pt
                f.write_str(if piece.connected_to(s.get(x + 1, y)) { " " } else { "|" })?;
            }

            f.write_str("\n")?;

            print_row_bottom_border(s, f, y)
        }

        fn print_row_bottom_border(s: &ColorableBoard, f: &mut fmt::Formatter, y: i16) -> fmt::Result {
            f.write_str(if s.get(0, y) == IndexedBoardState::Void {
                " "
            } else {
                "+"
            })?;

            for x in 0..s.width {
                let piece = s.get(x, y);

                f.write_str(if piece.connected_to(s.get(x, y + 1)) {
                    if piece.connected_to(s.get(x + 1, y)) && s.get(x, y + 1).connected_to(s.get(x + 1, y + 1)) {
                        "  "
                    } else {
                        " +"
                    }
                } else {
                    "-+"
                })?;
            }

            f.write_str("\n")
        }

        print_top_row_border(self, f)?;

        for y in 0..self.height {
            print_row(self, f, y)?;
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use polyomino::board::Board;
    use polyomino::point::Point;
    use polyomino::point::SimplePoint;
    use polyomino::polyomino::Polyomino;

    use crate::colorableboard::ColorableBoard;
    use crate::colorpolyomino::ColorPolyomino;
    use crate::colorpoint::ColorPoint;
    
    fn build_i() -> ColorPolyomino {
        let mut p = Vec::new();
        p.push(ColorPoint::new(0, 0));
        p.push(ColorPoint::new(0, 1));
        p.push(ColorPoint::new(0, 2));
        p.push(ColorPoint::new(0, 3));
        p.push(ColorPoint::new(0, 4));

        ColorPolyomino::new(p)
    }
    
    fn build_p() -> ColorPolyomino {
        let mut p = Vec::new();
        p.push(ColorPoint::new(0, 0));
        p.push(ColorPoint::new(1, 0));
        p.push(ColorPoint::new(2, 0));
        p.push(ColorPoint::new(0, 1));
        p.push(ColorPoint::new(1, 1));

        ColorPolyomino::new(p)
    }
    
    fn build_y() -> ColorPolyomino {
        let mut p = Vec::new();
        p.push(ColorPoint::new(1, 0));
        p.push(ColorPoint::new(0, 1));
        p.push(ColorPoint::new(1, 1));
        p.push(ColorPoint::new(1, 2));
        p.push(ColorPoint::new(1, 3));

        ColorPolyomino::new(p)
    }
    
    fn build_c() -> ColorPolyomino {
        let mut p = Vec::new();
        p.push(ColorPoint::new(0, 0));
        p.push(ColorPoint::new(0, 1));
        p.push(ColorPoint::new(1, 0));
        p.push(ColorPoint::new(0, 2));
        p.push(ColorPoint::new(1, 2));

        ColorPolyomino::new(p)
    }
    
    fn build_f() -> ColorPolyomino {
        let mut p = Vec::new();
        p.push(ColorPoint::new(1, 0));
        p.push(ColorPoint::new(0, 1));
        p.push(ColorPoint::new(1, 1));
        p.push(ColorPoint::new(1, 2));
        p.push(ColorPoint::new(2, 2));

        ColorPolyomino::new(p)
    }

    fn make_colorable_board() -> ColorableBoard {
        let mut b = Board::new(5, 5);

        let i = build_i();
        b.add_polyomino(&i, &SimplePoint::new(0, 0));
        let p = build_p();
        b.add_polyomino(&p, &SimplePoint::new(1, 0));
        let y = build_y();
        b.add_polyomino(&y, &SimplePoint::new(3, 0));
        let c = build_c();
        b.add_polyomino(&c, &SimplePoint::new(1, 2));
        let f = build_f();
        b.add_polyomino(&f, &SimplePoint::new(2, 2));

        ColorableBoard::new(&b)
    }

    #[test]
    fn same_polyomino() {
        let colorable_board = make_colorable_board();

        assert!(colorable_board.get(0, 0).get_poly_idx() == colorable_board.get(0, 1).get_poly_idx());
    }
    
    #[test]
    fn diff_polyomino() {
        let colorable_board = make_colorable_board();

        assert!(colorable_board.get(0, 0).get_poly_idx() != colorable_board.get(1, 0).get_poly_idx());
    }

    #[test]
    fn insert_different() {
        let mut v = Vec::new();

        v.push(build_i());

        let p = build_p();
        
        assert!(ColorableBoard::find_insert(&mut v, &p) == 1);
    }
    
    #[test]
    fn insert_same() {
        let mut v = Vec::new();

        v.push(build_i());

        let i = build_i();
        
        assert!(ColorableBoard::find_insert(&mut v, &i) == 0);
    }
}

















