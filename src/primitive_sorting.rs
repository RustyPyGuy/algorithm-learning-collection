
use rand::Rng;
use rand::distributions::Uniform;

// Merge Sort with Vectors with BORROWING
// function returns NOTHING and modifies the input vector that is referenced.
pub fn merge_sort_vec_borrow(vec_in: &mut Vec<i32>) {
    let len = vec_in.len();
    if len > 1 {
        #[allow(unused_assignments)] // allocating memory for large vector.
        let mut vec_low: Vec<i32> = Vec::with_capacity(len/2+1);
        #[allow(unused_assignments)]
        let mut vec_high: Vec<i32> = Vec::with_capacity(len/2+1);
        #[allow(unused_assignments)]
        let mut merg_vec: Vec<i32> = Vec::with_capacity(len+1);
        vec_low = vec_in[0..(len)/2].to_vec();
        vec_high = vec_in[(len)/2..len].to_vec();
        // println!("Input vector length (borrow): {}, first vector: \
        // {:?}, length: {:?} - second vector {:?}, length: {:?}",len,&vec_low,\
        // vec_low.len(),&vec_high,&vec_high.len());
        merge_sort_vec_borrow(&mut vec_low);
        merge_sort_vec_borrow(&mut vec_high);
        merge(&mut vec_low, &mut vec_high, &mut merg_vec);
        // println!("merge vec: {:?}", merg_vec);
        *vec_in = merg_vec;
    }
    else if len == 1 {
       // println!("base case reached");
       let mut returnvec: Vec<i32> = Vec::new();
       returnvec.push(vec_in[0]);
        #[allow(unused_variables)]  // assigned to borrowed input value
       let vec_in = returnvec;
    }
    else {
       let returnvec: Vec<i32> = Vec::new();
       #[allow(unused_variables)]
       let vec_in = returnvec;
    }
    // merge two references to vectors and change third vector
    fn merge(vec_in1: &mut Vec<i32>, vec_in2: &mut Vec<i32>, vec_out: &mut Vec<i32>) {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let len_vec_in1: usize = vec_in1.len();
        let len_vec_in2: usize = vec_in2.len();
        loop {
            if i < len_vec_in1 && j < len_vec_in2 {
                if vec_in1[i] < vec_in2[j] {
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
                return;
            }
        }
    } //end merge function
}

// Merge Sort with Vectors with OWNERSHIP and ASSIGNMENTS
// function takes ownership of parameters and returns a new vector
pub fn merge_sort_vec_own(vec_in: Vec<i32>) -> Vec<i32> {
    let len = vec_in.len();
    if len > 1 {
        #[allow(unused_assignments)]
        let mut vec_high: Vec<i32> = Vec::with_capacity(len/2+1);// ignore warning
        #[allow(unused_assignments)]
        let mut vec_low: Vec<i32> = Vec::with_capacity(len/2+1); //ignore warning
        #[allow(unused_assignments)]
        let mut merg_vec: Vec<i32> = Vec::with_capacity(len+1);  //ignore warning
        vec_low = vec_in[0..(len)/2].to_vec();
        vec_high = vec_in[(len)/2..len].to_vec();
        vec_low = merge_sort_vec_own(vec_low);
        vec_high = merge_sort_vec_own(vec_high);
        merg_vec = merge(vec_low, vec_high);
        // println!("merge vec: {:?}", merg_vec);
        return merg_vec;
    }
    else if len == 1 {
       // println!("base case reached");
       // let mut returnvec: Vec<i32> = Vec::with_capacity(1);
       // returnvec.push(vec_in[0]);
       // return returnvec;
       return vec!(vec_in[0]);
    }
    else {
        return Vec::new();
    }
    // merge two references to vectors and output a vector
    fn merge(vec_in1: Vec<i32>, vec_in2: Vec<i32>) -> Vec<i32>{
        let mut i: usize = 0;
        let mut j: usize = 0;
        let len_vec_in1: usize = vec_in1.len();
        let len_vec_in2: usize = vec_in2.len();
        let mut vec_out: Vec<i32> = Vec::with_capacity(len_vec_in1+len_vec_in1+1);
        loop {
            if i < len_vec_in1 && j < len_vec_in2 {
                if vec_in1[i] < vec_in2[j] {
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
    } //end merge function
}

// Step through vector element by element to see if it is sorted.
pub fn check_sort_vec(vec_in: &Vec<i32>) -> bool {
    if vec_in.len() == 0 {
        return true;
    }
    for index in 0..vec_in.len()-1 {
        if vec_in[index] <= vec_in[index+1] {
            //continue
        }
        else {
            return false;
        }
    }
    return true;
}

// generate a vector with a specifed size and elements of random values to a max range
pub fn generate_random_vector(vec_size: i32, max_value: i32) -> Vec<i32>{
    if vec_size == 0 {
        return Vec::new();
    }
     if vec_size > 2_100_000_001 {
        // Two Billion One Hundred Million is the limit. darn close to 2^32/2
        panic!("Requested vector vec_size exceeds appoximate max allowable value of an i32.");
    }
    let vec_usize = vec_size as usize; // NOTE: no checking of bounds
    let mut out_vec: Vec<i32> = Vec::with_capacity(vec_usize+1);
    let between = Uniform::from(0..max_value);
    let mut rng = rand::thread_rng();
    for _ in 0..vec_size {
       out_vec.push(rng.sample(between));
    }
    return out_vec;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_merge_sort_ownership() {
    let r_vec2 = generate_random_vector(101_101, 10_000);
    let out1 = merge_sort_vec_own(r_vec2);
    assert!(check_sort_vec(&out1));
    }
    #[test]
    fn check_merge_sort_borrowing() {
    let mut r_vec2 = generate_random_vector(101_101, 10_000);
    merge_sort_vec_borrow(&mut r_vec2);
    assert!(check_sort_vec(&r_vec2));
    }
}
