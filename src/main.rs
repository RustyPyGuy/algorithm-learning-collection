/*
 * This project is an exercise of implementing primitive algorithms in Rust.
 * It was written to learn both algorithm implementation and Rust at the same time by
 * an advanced beginner programmer. It is not lost on me that I am reinventing many
 * wheels here.  That's somewhat the point.  I attempt use features specific to
 * Rust such as alternating between ownership and borrowing to test ideas. Please be
 * kind with critiques. :-)
 *
 * Selected algorithms are based on lessons and pseudocode from:
 *          Algoritms Illuminated by Tim Roughgarten
 * Where appropriate I give references to the book chapters. 
 *
 * This code is releaed under the GPL v 3.0 license.  It is meant for learning.
 */

mod primitive_sorting;
use primitive_sorting::*;
mod coordinate;
// use coordinate::*;
use std::time::{/*Duration,*/ Instant};

fn main() {
println!("Welcome to the sorting and coordinate function exercise.\n\nSorting coordinate pairs.");
for _ in 1..5{
    println!("---------------------------------------------------------");
    run_closest_coordinate_2d(1_000_000,90_000_000);
}
println!("\n---------------------------------------------------------\n
         Creating random vectors, timing the sorts, and checking the results. Here we go!");
run_sort_test_ownership(0,10);
run_sort_test_borrowing(0,10);
run_sort_builtin_slice_sort(0,10);
run_sort_builtin_unstable_sort(0,10);
println!("---------------------------------------------------------");
run_sort_test_ownership(1,10);
run_sort_test_borrowing(1,10);
run_sort_builtin_slice_sort(1,10);
run_sort_builtin_unstable_sort(1,10);
println!("---------------------------------------------------------");
run_sort_test_ownership(2,10);
run_sort_test_borrowing(2,10);
run_sort_builtin_slice_sort(2,10);
run_sort_builtin_unstable_sort(2,10);
println!("---------------------------------------------------------");
run_sort_test_ownership(5,10);
run_sort_test_borrowing(5,10);
run_sort_builtin_slice_sort(5,10);
run_sort_builtin_unstable_sort(5,10);

let mut vlength = 10;
let mut vrange = 100;
// run sorting tests doubling in size for each run.
while vlength < 5_250_000 {
    println!("---------------------------------------------------------");
    run_sort_test_ownership(vlength,vrange);
    run_sort_test_borrowing(vlength,vrange);
    run_sort_builtin_slice_sort(vlength,vrange);
    run_sort_builtin_unstable_sort(vlength,vrange);
    vlength = vlength * 2;
    vrange = vlength * 10;
    }
println!("\nEnd of program.")
} // end main()

///Test the use of ownership of variables and transfer of scope in a simple merge sort.
fn run_sort_test_ownership(vector_size: i32, value_range: i32) {
    let random_vec = primitive_sorting::generate_random_vector(vector_size, value_range);
    let start = Instant::now();
    let output = primitive_sorting::merge_sort_vec_own(random_vec);
    let duration = start.elapsed();
    if check_sort_vec(&output) == true {
    println!("vector len: {} sorted (Ownership fn). Sort time: {:?}",output.len(), duration);
    }
    else{
    println!("vector len: {} -NOT- successfully sorted. Run time of sort: {:?}",output.len(), duration);
    }
}

///Test the use of borrowing of variables and writing tot he same variable in a simple merge sort.
fn run_sort_test_borrowing(vector_size: i32, value_range: i32) {
    let mut random_vec = primitive_sorting::generate_random_vector(vector_size, value_range);
    let start = Instant::now();
    primitive_sorting::merge_sort_vec_borrow(&mut random_vec);
    let duration = start.elapsed();
    if check_sort_vec(&random_vec) == true {
    println!("vector len: {} sorted (Bowrrowing fn). Sort time: {:?}",random_vec.len(), duration);
    }
    else{
    println!("vector len: {} -NOT- successfully sorted. Run time of sort: {:?}",random_vec.len(), duration);
    }
}

/// Run a timing test of the builtin slice sort method using a stable Timsort algorithm.
fn run_sort_builtin_slice_sort(vector_size: i32, value_range: i32) {
    let mut random_vec = primitive_sorting::generate_random_vector(vector_size, value_range);
    let random_vec_slice = &mut random_vec[..];
    let start = Instant::now();
    random_vec_slice.sort();
    let duration = start.elapsed();
    let check_vec = random_vec_slice.to_vec();
    if check_sort_vec(&check_vec) == true {
    println!("slice len: {} sorted (builtin slice.sort). Sort time: {:?}",random_vec.len(), duration);
    }
    else{
    println!("slice len: {} -NOT- successfully sorted. Run time of sort: {:?}",random_vec.len(), duration);
    }
}

/// Run a timing test of the builtin slice sort method using an unstable sorting algorithm.
fn run_sort_builtin_unstable_sort(vector_size: i32, value_range: i32) {
    let mut random_vec = primitive_sorting::generate_random_vector(vector_size, value_range);
    let random_vec_slice = &mut random_vec[..];
    let start = Instant::now();
    random_vec_slice.sort_unstable();
    let duration = start.elapsed();
    let check_vec = random_vec_slice.to_vec();
    if check_sort_vec(&check_vec) == true {
    println!("slice len: {} sorted (builtin slice.unstable_sort). Sort time: {:?}",
    random_vec.len(), duration);
    }
    else{
    println!("slice len: {} -NOT- successfully sorted. Run time of sort: {:?}",
             random_vec.len(), duration);
    }
}

/// Run a timing test on the closest cooridnate algorithm.
fn run_closest_coordinate_2d(vector_size: i32, value_range: i32){
    let mut rand_vec = coordinate::generate_random_vector_2d(vector_size,value_range);
    // let rand_vec = generate_random_pairs(vector_size,value_range);
    let coord_struct = coordinate::preprocess_coordinate_vector(&mut rand_vec);
    let start = Instant::now();
    let final_pair = coordinate::closest_pair_2d(coord_struct.x_sorted, coord_struct.y_sorted);
    let duration = start.elapsed();
    println!("Coord vec length: {} min/max: {} run time: {:?}\nfinal pair: {:?}, distance: {:}",
             vector_size, value_range, duration, final_pair, coordinate::distance_pair(&final_pair));
}
