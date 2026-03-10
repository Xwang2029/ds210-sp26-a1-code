git use std::{ops::Index, time::Instant};

fn mid_by_pointer(v: *const Vec<i32>) -> i32 {
    unsafe {
        let length = (*v).len();

        let vec_ref = &*v;

        return vec_ref[length / 2];
    }
}

fn mid_by_copy(v: Vec<i32>) -> i32 {
    let length = v.len();
    return v[length / 2];
}

fn main() {
    let mut vec = Vec::new();
    for i in 0..200000000 {
        vec.push(i);
    }

    // By pointer.
    let start_time1 = Instant::now();
    let mid1 = mid_by_pointer(&vec as *const Vec<i32>);
    let time1 = start_time1.elapsed();
    println!("By pointer returned {} and took {:?}", mid1, time1);

    // By copy.
    let start_time2 = Instant::now();
    let mid2 = mid_by_copy(vec.clone());
    let time2 = start_time2.elapsed();
    println!("By copy returned {} and took {:?}", mid2, time2);

    // We can can continue to use the vector afterwards to do other things.
    println!("The first and last elements in the vector are {} and {}", vec[0], vec[vec.len() - 1]);
}
