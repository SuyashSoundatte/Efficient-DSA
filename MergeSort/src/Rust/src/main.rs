// src/main.rs
mod merge_sort;
mod thread_merge_sort;

use merge_sort::MergeSort;
use thread_merge_sort::ParallelMergeSort;
use rand::Rng;
use std::time::Instant;

fn main() {
    // warning if your machine is having only 8 cores then it will takes lot of time or it might hang your machine so just reduce the array element size
    const SIZE: usize = 100_000_000;

    let mut nums = vec![0; SIZE];
    let mut nums1 = vec![0; SIZE];

    let mut rng = rand::thread_rng();
    for i in 0..SIZE {
        nums[i] = rng.gen_range(0..100_000_000);
        nums1[i] = rng.gen_range(0..100_000_000);
    }

    let mut mergesort = MergeSort::new(&mut nums);
    let start = Instant::now();
    mergesort.sort();
    let mergesort_duration = start.elapsed();
    println!("MergeSort algorithm time taken: {:.2} seconds", mergesort_duration.as_secs_f64());

    let mut parallel_mergesort = ParallelMergeSort::new(&mut nums1);
    let start = Instant::now();
    parallel_mergesort.sort();
    let parallel_mergesort_duration = start.elapsed();
    println!("Parallel MergeSort algorithm time taken: {:.2} seconds", parallel_mergesort_duration.as_secs_f64());
}
