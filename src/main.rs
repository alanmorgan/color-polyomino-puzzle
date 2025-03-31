mod colorpoint;
mod colorpolyomino;
mod colorableboard;
mod boardcolorer;
mod utils;

use std::env;

use polyomino::board::Board;
use polyomino::polyomino::Polyomino;
use polyomino::solver::Solver;
use polyomino::utils as poly_utils;
use polyomino::utils::Restrictions;
use polyomino::utils::PredefinedPolyominoes;

use crate::colorpolyomino::ColorPolyomino;
use crate::colorableboard::ColorableBoard;
use crate::boardcolorer::color_board;
use crate::utils::*;

// This was originally going to be much more complicated
//
// The idea is to get a set of pentomioes that are colored and form
// them into a rectange such that the colors themselves form a
// complete set of pentominoes.
//
// Take a solution
// Four color it (typically this can be done with three colors)
// Overlay that solution on a different solution
// That different solution now has a set of colored pentominoes
// Use those pentominoes and see how many solutions there are such
//   that there are a complete set of pentominoes in the colors
//
// The original idea was to do a search. Take every solution and apply
// it to every other solution (cull out uninteresting cases where you
// end up with a mono-colored pentomino on the target solution).
//
// This turns out to be unnecssary. As long as you pick two solutions such
// that you don't end up with a mono-colored pentomino after the overlay,
// you'll probaby end up with just one viable solution
//
// The only other issue is picking a board with a nice initial coloring
// Everything can be four-colored (obivously), but the algorithm is too
// good and usually (?) only one polyomino needs the fourth color
//
// find_nice_coloring will look for a coloring that is perfectly balanced
// (a surprising number are) and then you are off to the races 

struct Config {
    pub xsize: i16,
    pub ysize: i16,
    pub base_solution_number: Option<usize>,
    pub target_solution_number: Option<usize>
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage:\npolycolorpuzzles xsize ysize [base solution #] [target solution #]\n");
        println!("polycolorpuzzle xsize ysize\n\tenumerate boards of that size that can be nicely colored");
        println!("polycolorpuzzle xsize ysize base\n\tprint out board 'base' colored");
        println!("polycolorpuzzle xsize ysize base target\n\tcolor 'target' with 'base' and check for suitability");
        return;
    }

    let xsize = args[1].parse::<i16>().unwrap();
    let ysize = args[2].parse::<i16>().unwrap();

    let base_solution_number = if args.len() >= 4 {
        Some(args[3].parse::<usize>().unwrap())
    } else {
        None
    };

    let target_solution_number = if args.len() > 4 {
        Some(args[4].parse::<usize>().unwrap())
    } else {
        None
    };

    let config = Config {xsize, ysize, base_solution_number, target_solution_number};
    
    if let Ok(mut polyominoes) = poly_utils::get_polyominoes::<ColorPolyomino>(PredefinedPolyominoes::Pentominoes) {

        polyominoes.iter_mut().enumerate().for_each(|(id, p)| p.set_id(id));
        
        let all_polyominoes = poly_utils::build_variations(&polyominoes, Restrictions::RectangularSymmetry);

        let mut b = get_board(&config);

        let mut solver = Solver::new(&mut b, &all_polyominoes);

        println!("Generating solutions");
        let solutions:Vec<ColorableBoard> = solver.solve().iter().map(|s| ColorableBoard::new(s)).collect();

        if config.base_solution_number.is_none() {
            find_nice_colorings(&solutions);
        } else if config.target_solution_number.is_none() {
            let mut base_solution = solutions[config.base_solution_number.unwrap()].clone();
            color_board(&mut base_solution);
            println!("{}", base_solution);
        } else {
            let mut base_solution = solutions[config.base_solution_number.unwrap()].clone();
            color_board(&mut base_solution);
            
            let target_solution = &mut solutions[config.target_solution_number.unwrap()].clone();
            
            build_single_solution_variations(&config, &base_solution, target_solution, Orientation::Normal, &all_polyominoes);
        }
    } else {
        panic!("Can't find polyomino file");
    }
}

fn get_board<'a>(config: &Config) -> Board<'a, ColorPolyomino> {
    Board::new(config.xsize, config.ysize)
}

// Looks for a coloring that is well balanced.
fn find_nice_colorings(solutions: &Vec<ColorableBoard>) {
    for (i, soln) in solutions.iter().enumerate() {
        let mut base_solution = soln.clone();
        color_board(&mut base_solution);
        
        let cnt = color_count(&base_solution);
        
        if cnt.len() == 3 && cnt.iter().all(|v| *v == 4*5) {
            println!("Solution {} has a nice 3 coloring", i);
        } else if cnt.iter().all(|v| *v == 3*5) {
            println!("Solution {} has a nice 4 coloring", i);
        }
    }
}

fn build_all_solution_variations<P: Polyomino>(config: &Config, base_solution: &ColorableBoard, all_solutions: &Vec<ColorableBoard>, all_polyominoes: &Vec<Vec<P>>) {
    let mut colored_solution: ColorableBoard = base_solution.clone();
            
    color_board(&mut colored_solution);
    
    for target_soln in all_solutions {
        let mut target_soln_mut: ColorableBoard = target_soln.clone();
        
        build_single_solution_variations(config, &colored_solution, &mut target_soln_mut, Orientation::Normal, &all_polyominoes);
        
        build_single_solution_variations(config, &colored_solution, &mut target_soln_mut, Orientation::OneEighty, &all_polyominoes);
        
        build_single_solution_variations(config, &colored_solution, &mut target_soln_mut, Orientation::FlipHorizontally, &all_polyominoes);
        
        build_single_solution_variations(config, &colored_solution, &mut target_soln_mut, Orientation::FlipOneEighty, &all_polyominoes);
    }
}

fn build_single_solution_variations<P: Polyomino>(config: &Config, base_solution: &ColorableBoard, target_solution: &mut ColorableBoard, orientation: Orientation, all_polyominoes: &Vec<Vec<P>>) {
    println!("Base solution\n{}", base_solution);
    
    overlay(base_solution, target_solution, orientation);

    println!("Target solution\n{}", target_solution);
    
    // After imposing a coloring on another board, if any of the
    // polyominoes on that board is mono-colored, we reject the
    // solution as being boring
    if !has_single_color_polyomino(&target_solution) {

        // Take the polyominoes that make up this solution and build the variations of those
        let colored_polys = poly_utils::build_variations(&target_solution.polyominoes,
                                                         Restrictions::RectangularSymmetry);

        // Build solutions with those polyominos
        let mut board = get_board(config);
        let mut solver = Solver::new(&mut board, &colored_polys);

        println!("Solving...");
        let solutions = solver.solve();

        println!("Found {} posible solutions. Checking...", solutions.len());
        
        // Now check to see if any of those solutions has a connected
        // blob of color in the pattern of every polyomino

        let mut valid_solution_variations = Vec::new();
        
        for solution in solutions {
            let colorable_solution = ColorableBoard::new(solution);
            if has_all_polyomino_patterns(&colorable_solution, &all_polyominoes) {
                valid_solution_variations.push(colorable_solution.clone());
            }
        }

        if !valid_solution_variations.is_empty() {
            println!("\n\n{} valid solution(s)", valid_solution_variations.len());

            if valid_solution_variations.len() < 10 {
                valid_solution_variations.iter().for_each(|s| println!("{}", s));
            }
        }
    } else {
        println!("Target has mono-colored polyomino. Try a different one");
    }
}












