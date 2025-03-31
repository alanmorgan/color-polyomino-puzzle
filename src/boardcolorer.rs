use heuristic_graph_coloring::*;
use std::cmp;
use std::collections::HashSet;
use crate::colorpolyomino::ColorPolyomino;
use crate::colorableboard::ColorableBoard;
use crate::colorableboard::IndexedBoardState;

#[allow(dead_code)]

pub fn color_board(b: &mut ColorableBoard) {
    let colors: Vec<usize> = find_coloring(b);

    for x in 0..b.width {
        for y in 0..b.height {
            if let IndexedBoardState::Full(poly_idx, pt_idx, _x, _y) = b.get(x, y) {
                let poly:&mut ColorPolyomino = b.get_polyomino_mut(poly_idx);

                let color = format!("{:x}", colors[poly.get_id()]).chars().last().unwrap();
                
                poly[pt_idx].set_color(color);
            }
        }
    }
}

fn find_coloring(b: &ColorableBoard) -> Vec<usize>{
    let mut adjacencies = HashSet::new();

    // set the id on all the polys
    
    for x in 0..b.width {
        for y in 0..b.height {
            mark_as_adjacent(b, &b.get(x, y), &b.get(x+1, y), &mut adjacencies);
            mark_as_adjacent(b, &b.get(x, y), &b.get(x, y+1), &mut adjacencies);
        }
    }

    let mut graph = VecVecGraph::new(adjacencies.len());
    for (poly1, poly2) in adjacencies {
       graph.add_edge(poly1, poly2);
    }

    let result = color_rlf(&graph);

    result
}

fn mark_as_adjacent(b: &ColorableBoard, b1: &IndexedBoardState, b2: &IndexedBoardState, adjacencies: &mut HashSet<(usize, usize)>) {
    if let IndexedBoardState::Full(p1_idx, _pt, _x, _y) = b1 {
        if let IndexedBoardState::Full(p2_idx, _pt, _x, _y) = b2 {
            if b.get_polyomino(*p1_idx).get_id() != b.get_polyomino(*p2_idx).get_id() {
                // To avoid adding an edge twice, normalize them
                let edge = (cmp::min(b.get_polyomino(*p1_idx).get_id(), b.get_polyomino(*p2_idx).get_id()),
                            cmp::max(b.get_polyomino(*p1_idx).get_id(), b.get_polyomino(*p2_idx).get_id()));
                
                adjacencies.insert(edge);
            }
        }
    }
}






