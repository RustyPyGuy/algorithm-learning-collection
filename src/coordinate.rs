/*
 * Functions for sorting and calulating points with 2 dimensions.
 */

// constants for sizes of test vectors created
const _TEST_VEC: i32 = 501;
const _TEST_VEC_RANGE: i32 = 10000;

extern crate integer_sqrt;
use integer_sqrt::IntegerSquareRoot;
// use std::convert::TryFrom;
use rand::Rng;
use rand::distributions::Uniform;

pub struct PreprocessedCords {
    pub x_sorted: Vec<[i32;2]>,
    pub y_sorted: Vec<[i32;2]>,
}

/// Processes and input vector of coordinates to prepare it for the closest pair algorithm. Checks
/// that there are an even number of elements and outputs a struct of the elements x-sorted and
/// y-sorted.
pub fn preprocess_coordinate_vector(input_coord_vec: &mut Vec<[i32;2]>) -> PreprocessedCords {
// takes an input vector of 2d coordinates and sorts and doubles for later algos
//sort by x coordinate, y coordinate, and calculates the delta value
        let len;
        // if the vector length is odd, duplicate the last element to make it an even length.
        if input_coord_vec.len() % 2 == 1 {
            len = input_coord_vec.len()+1;
            input_coord_vec.push(input_coord_vec[len-2]);
        }
        // else{
        //     len = input_coord_vec.len();
        // }
let sorted_x_pairs: Vec<[i32;2]> = merge_sort_vector_2d(&input_coord_vec, 0);
let sorted_y_pairs: Vec<[i32;2]> = merge_sort_vector_2d(&input_coord_vec, 1);
    // return a struct with two vectors.
    let return_struct = PreprocessedCords {
        x_sorted: sorted_x_pairs,
        y_sorted: sorted_y_pairs,
    };  // semicolon! 
    return return_struct;
}

/// Computes the closest two coordinate pairs in a vector of coordinate pairs. This is the core of
/// the divide-and-conquor algorithm using recursiveness for efficient searching as described in
/// the text chapter 3.4 of Algorithms Illuminated.  Parameters are a vector sorted by x
/// coordinates and the same vector sorted by y coordinates. Returns a vector with a 2 elements
/// with the matching pair. Input order is not preserved. Overlapping pairs are treated as
/// duplicates and not matched.
pub fn closest_pair_2d(x_sorted_vec: Vec<[i32;2]>, y_sorted_vec: Vec<[i32;2]>) -> Vec<[i32;2]> {
    let len = x_sorted_vec.len();
    // base recursion case of a 1 or 2 element vector is input
    if len == 1 || len == 2 {
        return x_sorted_vec;
    }
    // Split the vector in half sorted by x and y coordinates.
    let x_sorted_vec_left: Vec<[i32;2]> = x_sorted_vec[0..(len)/2].to_vec();
    let x_sorted_vec_right: Vec<[i32;2]> = x_sorted_vec[(len)/2..len].to_vec();
    let y_sorted_vec_left: Vec<[i32;2]> = y_sorted_vec[0..(len)/2].to_vec();
    let y_sorted_vec_right: Vec<[i32;2]> = y_sorted_vec[(len)/2..len].to_vec();
    // Recurse to find the closest pair in each half and calculate the distance.  using the
    // distance squared calculation and comparison to avoid performing a square root.
    let best_left: Vec<[i32;2]> = closest_pair_2d(x_sorted_vec_left, y_sorted_vec_left);
    let distance_squared_left = distance_squared(&best_left);
    let best_right: Vec<[i32;2]> = closest_pair_2d(x_sorted_vec_right, y_sorted_vec_right);
    let distance_squared_right = distance_squared(&best_right);
    // Evalute which half has the lowest distance.
    let best_distance_squared;
    if distance_squared_left < distance_squared_right {
        best_distance_squared = distance_squared_left;
    }
    else {
        best_distance_squared = distance_squared_right;
    }
    // Evaluate whether the closest pair that may be split between left and right vectors.
    // let best_split: Vec<[i32;2]> = closest_split_pair_2d(x_sorted_vec, y_sorted_vec, best_distance_squared);
    // TODO: fix borrowing in closest_split_pair_2d_lr function so that these variables are not redeclared.
    let x_sorted_vec_left: Vec<[i32;2]> = x_sorted_vec[0..(len)/2].to_vec();
    let x_sorted_vec_right: Vec<[i32;2]> = x_sorted_vec[(len)/2..len].to_vec();
    let y_sorted_vec_left: Vec<[i32;2]> = y_sorted_vec[0..(len)/2].to_vec();
    let y_sorted_vec_right: Vec<[i32;2]> = y_sorted_vec[(len)/2..len].to_vec();
    let best_split: Vec<[i32;2]> = closest_split_pair_2d_lr(x_sorted_vec_left, y_sorted_vec_left, x_sorted_vec_right, y_sorted_vec_right, best_distance_squared);
    // Compare the left, right, and split pairs for which is the best (closest) and return the
    // pair.
    // TODO: There are some inefficiencies with the below function repeating distance
    // calculations already performed.
    let best_pair = closest_two_pairs(best_left, best_right);
    return closest_two_pairs(best_pair, best_split); // if distances are equal, the split pair will be returned.

    /// function to separate the left half of a vector and the right half of the vector and return
    /// the closest split pair that bridgest this boundary.  This is integral to the closest pari
    /// algorithm.
    fn closest_split_pair_2d_lr(x_sorted_vec_left: Vec<[i32;2]>, y_sorted_vec_left: Vec<[i32;2]>, x_sorted_vec_right: Vec<[i32;2]>, y_sorted_vec_right: Vec<[i32;2]>, distance_squared_in: i64) -> Vec<[i32;2]> {
// NOTE: FAILURE: Algoritm failure.  Something is missing from this. it does not always find the closest
// split pair and fails tests.
        let len = x_sorted_vec_left.len();
        let mut variance_index: usize = 0;

        let mut distance_squared_test_pairs: i64 = 0;
        // determine number of indexes to expand from median to not exceed distance
        while distance_squared_in > distance_squared_test_pairs {
            let mut test_pair_vector: Vec<[i32;2]> = Vec::new();
            // check that there is no negative index or index beyond length.
            if len == variance_index  /*|| len+variance_index == len-1 */{
                break;
            }
            variance_index += 1;
            test_pair_vector.push(x_sorted_vec_left[len-variance_index]);
            test_pair_vector.push(x_sorted_vec_right[variance_index-1]);
            distance_squared_test_pairs = distance_squared(&test_pair_vector);
        }
        // // reassemble vector in order with the smaller subset of x-sorted points, unsorted.
        // let mut distance_bound_vec: Vec<[i32;2]> = Vec::new();
        // for i in 1..variance_index {
        //     distance_bound_vec.push(x_sorted_vec_left[len-i]);
        //     distance_bound_vec.push(x_sorted_vec_right[i-1]);
        // }
        // reassemble vector in order with the smaller subset of y-sorted points, unsorted.
        let mut distance_bound_vec: Vec<[i32;2]> = Vec::new();
        for i in 1..variance_index {
            distance_bound_vec.push(y_sorted_vec_left[len-i]);
            distance_bound_vec.push(y_sorted_vec_right[i-1]);
        }
        // per algorithm definition, sort by y element.
        distance_bound_vec = merge_sort_vector_2d(&distance_bound_vec, 1);
        let len2 = distance_bound_vec.len();
        let mut best_pair: Vec<[i32;2]> = Vec::with_capacity(2);
        // let mut best_pair_distance_squared: i64 = distance_squared_in;
        let mut best_pair_distance_squared: i64 = i64::MAX;
        let test_max: usize;
        if len2 == 0 {
            test_max = 0;
        }
        else if len2 < 8 {
            test_max = len2-1;
        }
        else {
            test_max = 7;
        }
        for i in 0..test_max {
            for j in 0..test_max {
                // let test_vec: Vec<[i32;2]> =  distance_bound_vec[i];
                let mut pair_test_vec: Vec<[i32;2]> = Vec::new();
                pair_test_vec.push(distance_bound_vec[i]);
                pair_test_vec.push(distance_bound_vec[j]);

                if distance_squared(&pair_test_vec) < best_pair_distance_squared {
                    best_pair_distance_squared = distance_squared(&pair_test_vec);
                    best_pair = pair_test_vec;
                }
            }
        }
    // println!("test_max: {} best pair: {:?}", test_max, best_pair);
    return best_pair;
        }
    }

    /// Compares two sets of pairs and returns the vector of two pairs that are closer.
    fn closest_two_pairs(pair1: Vec<[i32;2]>, pair2: Vec<[i32;2]>) -> Vec<[i32;2]> {
        let d_squared_1 = distance_squared(&pair1);
        let d_squared_2 = distance_squared(&pair2);
        // if distances are equal, then the second pair is always returned.
        if d_squared_1 < d_squared_2 {
            return pair1;
        }
        return pair2;
    }

    /// takes two coordinates and performs simple arithmetic for the distance squared.
    /// Returns the maximum integer value if the two pairs are equal or an incorrect number of pairs are given.
    fn distance_squared(pair: &Vec<[i32;2]>) -> i64 {
        if pair.len() != 2{
            return i64::MAX;
        }
        // returns max value to reduce tests in other functions if comparison points are identical.
        if pair[0] == pair[1]{
            return i64::MAX;
        }
        // Rust doesn't have built-in exponentiation without
        // importing a library. Return C^2 = (Ax-Bx)^2 + (Ay-By)^2
        return (pair[0][0] as i64-pair[1][0] as i64)*(pair[0][0] as i64 -pair[1][0] as i64) +
                        (pair[0][1] as i64-pair[1][1] as i64)*(pair[0][1] as i64-pair[1][1] as i64);
    }

/// returns the integer distance of two pairs in a 2D coordinate plane.  Returns max value if the
/// same point is given for both elements or more than 2 elements are given. This function is not
/// used in the algoritm but it is useful for reviewing output.
pub fn distance_pair(pair: &Vec<[i32;2]>) -> i64{
    // let distance_squared1 = distance_squared(&pair);
    return distance_squared(&pair).integer_sqrt();
}

/// Merge Sort of a Vector containing of 2 element arrays intended to represent a vector of fixed
/// 2D coordinates.  Parameters are an input of a borrowed Vector, and an key of which array
/// value (x or y) to use as a sorting key.  Returns a sorted Vector.
pub fn merge_sort_vector_2d(vec_in: &Vec<[i32;2]>, key: usize) -> Vec<[i32;2]> {
    if key > 1 {
        panic!("invalid key value. Must be 0 or 1.");
    }
    let len = vec_in.len();
    if len > 1 {
        #[allow(unused_assignments)] // allocating memory for a vector to be written.
        let mut vec_high: Vec<[i32;2]> = Vec::with_capacity(len);
        #[allow(unused_assignments)]
        let mut vec_low: Vec<[i32;2]> = Vec::with_capacity(len);
        #[allow(unused_assignments)]
        let mut merg_vec: Vec<[i32;2]> = Vec::with_capacity(len);
        vec_low = vec_in[0..(len)/2].to_vec();
        vec_high = vec_in[(len)/2..len].to_vec();
        vec_low = merge_sort_vector_2d(&vec_low, key);
        vec_high = merge_sort_vector_2d(&vec_high, key);
        merg_vec = merge_element(vec_low, vec_high, key);
        return merg_vec;
    }
    else if len == 1 {
       let returnvec: Vec<[i32;2]> = vec_in.to_vec();
       return returnvec;
    }
    else {
        return Vec::new();
    }
    // merge two references to vectors and output a vector
    fn merge_element(vec_in1: Vec<[i32;2]>, vec_in2: Vec<[i32;2]>, key: usize) -> Vec<[i32;2]> {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let len_vec_in1: usize = vec_in1.len();
        let len_vec_in2: usize = vec_in2.len();
        let mut vec_out: Vec<[i32;2]> = Vec::with_capacity(len_vec_in1+len_vec_in2);
        loop {
        // let len_vec_in1: usize = vec_in1.len();
            if i < len_vec_in1 && j < len_vec_in2 {
                if vec_in1[i][key] < vec_in2[j][key] {
                vec_out.push(vec_in1[i]);
                i = i+1;
                }
                else{
                    vec_out.push(vec_in2[j]);
                    j = j+1;
                }
            }
            else if i < len_vec_in1 {
                vec_out.push(vec_in1[i]);
                i = i+1;
            }
            else if j < len_vec_in2 {
                vec_out.push(vec_in2[j]);
                j = j+1;
            }
            else{
                return vec_out;
            }
        }
    } // end merge function.
} // end merge_sort_vector_2d function.

/// Generates random coordinate pairs into a vector with specificed size and min and max values.
#[allow(dead_code)]
pub fn generate_random_pairs(vec_size: i32, max_value: i32) -> Vec<[i32;2]> {
    if vec_size == 0 {
        return Vec::new();
    }
     if vec_size > i32::MAX - 2 {
        // Two Billion One Hundred Million-ish is the limit.
        panic!("Requested vector vec_size exceeds appoximate max allowable value of an i32.");
    }
    let vec_usize = vec_size as usize; // NOTE: no checking of bounds
    let mut out_vec: Vec<[i32;2]> = Vec::with_capacity(vec_usize+1);
    let between = Uniform::from(-max_value..max_value);
    let mut rng = rand::thread_rng();
    for _ in 0..vec_size {
       out_vec.push([rng.sample(between),rng.sample(between)]);
    }
    out_vec.to_vec();
    return out_vec;
}

/// An exhaustive search of a vector of cooridnate pairs to find the closest pair using a
/// simplistic double loop.  Results in an O(n^2) algoritm.
#[allow(dead_code)]
pub fn closest_pair_exhaustive(pair_in: Vec<[i32;2]>) -> Vec<[i32;2]> {
    let len: usize = pair_in.len();
    if len == 2 {
        return pair_in;
    }
    if len < 1 {
        return Vec::new();
    }
    let mut distance_squared_test_pair: i64;
    let mut distance_squared_best_pair: i64 = i64::MAX;
    let mut winning_pair: Vec<[i32;2]>  = Vec::new();
    let mut test_pair: Vec<[i32;2]>; // = Vec::new();
    for i in 0..len {
        for j in 0..len {
            test_pair = vec!(pair_in[i],pair_in[j]);
            distance_squared_test_pair = distance_squared(&test_pair);
            if distance_squared_test_pair < distance_squared_best_pair {
                distance_squared_best_pair = distance_squared_test_pair;
                winning_pair = test_pair;
            }
        }
    }
    return winning_pair;
}
/*
 * Functions to support testing. Data generation and checking.
 */

#[allow(dead_code)]
pub fn check_sort_vec_2d(vec_in: &Vec<[i32;2]>, key: usize) -> bool {
    if key > 1 {
        panic!("invalid key. must be 0 or 1.")
    }
    let len = vec_in.len() as usize;
    let mut index: usize = 0;
    while index < len-1 {
        // test_vec2 = vec_in[index+1].to_vec();
        if vec_in[index][key] <= vec_in[index+1][key] {
            //continue
        }
        else {
            return false;
        }
        index = index + 1;
    }
    return true;
}

pub fn generate_random_vector_2d(vec_size: i32, max_coord_value: i32) -> Vec<[i32;2]>{
 if vec_size > 2_100_000_001 {
        // Two Billion One Hundred Million is the limit. darn close to 2^32/2
        panic!("Requested vector vec_size exceeds appoximate max allowable value of an i32.");
    }
    let vec_usize = vec_size as usize; // NOTE: no checking of bounds
    let mut out_vec:  Vec<[i32;2]> = Vec::with_capacity(vec_usize);
    let between = Uniform::from(-max_coord_value..max_coord_value);
    let mut rng = rand::thread_rng();
    for _ in 0..vec_size {
       out_vec.push([rng.sample(between),rng.sample(between)]);
    }
    return out_vec;
}

/*
 * Tests
 */
#[cfg(test)]
mod tests {
    use super::*;
    /// Switch positions of two pairs in a 2-element cooridnate vector.
    fn invert_pair(pair_in: Vec<[i32;2]>) -> Vec<[i32;2]> {
        let len = pair_in.len();
        if len != 2 {
            return Vec::new();
        }
        let mut vec_out: Vec<[i32;2]> = Vec::new();
        vec_out.push(pair_in[1]);
        vec_out.push(pair_in[0]);
        return vec_out;
    }

    #[test]
    fn sort_by_x_key_in_small_manual_vector(){ //for an eyeball comparison
        let test1_vec: Vec<[i32;2]> = vec![[1,10],[10,1],[5,5],[2,7],[100,99],[99,100],[20,4]];
        let sorted_x_pairs: Vec<[i32;2]> = merge_sort_vector_2d(&test1_vec, 0);
        assert_eq!(sorted_x_pairs,vec![[1,10],[2,7],[5,5],[10,1],[20,4],[99,100],[100,99]]);
    }

    #[test]
    fn sort_by_y_key_in_small_manual_vector(){
        let test1_vec: Vec<[i32;2]> = vec![[1,10],[10,1],[5,5],[2,7],[100,99],[99,100],[20,4]];
        let sorted_x_pairs: Vec<[i32;2]> = merge_sort_vector_2d(&test1_vec, 1);
        assert_eq!(sorted_x_pairs,vec![[10,1],[20,4],[5,5],[2,7],[1,10],[100,99],[99,100]]);
    }

    #[test]
    fn sort_by_x_key_in_large_random_vector(){
        let test1_vec: Vec<[i32;2]> = generate_random_vector_2d(_TEST_VEC, _TEST_VEC_RANGE);
        let sorted_x_pairs: Vec<[i32;2]> = merge_sort_vector_2d(&test1_vec, 0);
        println!("vector len: {} sorted? {:?}",sorted_x_pairs.len(), sorted_x_pairs);
        assert!(check_sort_vec_2d(&sorted_x_pairs ,0));
    }

    #[test]
    fn sort_by_y_key_in_large_random_vector(){
        let test1_vec: Vec<[i32;2]> = generate_random_vector_2d(_TEST_VEC, _TEST_VEC_RANGE);
        let sorted_x_pairs: Vec<[i32;2]> = merge_sort_vector_2d(&test1_vec, 1);
        println!("vector len: {} sorted? {:?}",sorted_x_pairs.len(), sorted_x_pairs);
        assert!(check_sort_vec_2d(&sorted_x_pairs ,1));
    }

    #[test]
    fn test_closest_pair_exhuastive_small_vector(){
        let test1_vec: Vec<[i32;2]> = vec![[1,10],[10,1],[101,100],[5,5],[2,7],[100,98],[99,100],[100,100],[20,4]];
        let closest_pair: Vec<[i32;2]> = closest_pair_exhaustive(test1_vec);
        let expected_pair = vec![[100,100],[101,100]];
        assert!(closest_pair == expected_pair || closest_pair == invert_pair(expected_pair));
    }

    #[test]
    fn test_closest_pair_compare_efficient_exhaustive(){
        #[allow(unused_assignments)] // allocating memory for a vector to be written.
        let mut test1_vec: Vec<[i32;2]> = Vec::with_capacity(_TEST_VEC as usize);
        let mut fail_counter = 0;
        let runs: i32 = 10;
        println!("Number of test runs on random vector data: {}", runs);
        for _ in 0..runs{
            test1_vec = generate_random_vector_2d(_TEST_VEC, _TEST_VEC_RANGE);
            let coord_struct = preprocess_coordinate_vector(&mut test1_vec);
            let closest_pair_exhaustive: Vec<[i32;2]> = closest_pair_exhaustive(test1_vec);
            let closest_pair_efficient: Vec<[i32;2]> = closest_pair_2d(coord_struct.x_sorted, coord_struct.y_sorted);
            println!("results - exhaustive: {:?} d: {} | efficient: {:?} d: {}", 
                     closest_pair_exhaustive, distance_pair(&closest_pair_exhaustive), 
                     closest_pair_efficient, distance_pair(&closest_pair_efficient));
            if !(closest_pair_exhaustive == closest_pair_efficient || 
                closest_pair_exhaustive == invert_pair(closest_pair_efficient)) {
                fail_counter += 1
            }
        }
        println!("Fail counts {} of {} runs", fail_counter, runs);
        assert!(fail_counter==0);

        // let mut test1_vec: Vec<[i32;2]> = generate_random_vector_2d(_TEST_VEC, _TEST_VEC_RANGE);
        // let coord_struct = preprocess_coordinate_vector(&mut test1_vec);
        // let closest_pair_exhaustive: Vec<[i32;2]> = closest_pair_exhaustive(test1_vec);
        // let closest_pair_efficient: Vec<[i32;2]> = closest_pair_2d(coord_struct.x_sorted, coord_struct.y_sorted);
        // println!("results - exhaustive: {:?} d: {} | efficient: {:?} d: {}", closest_pair_exhaustive, distance_pair(&closest_pair_exhaustive), closest_pair_efficient, distance_pair(&closest_pair_efficient));
        // assert!(closest_pair_exhaustive == closest_pair_efficient || closest_pair_exhaustive == invert_pair(closest_pair_efficient));
    }

}

