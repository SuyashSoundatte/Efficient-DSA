use std::thread;

pub struct ParallelMergeSort {
    nums: Vec<i32>, // Changed to Vec<i32> to own the data
}

impl ParallelMergeSort {
    pub fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    pub fn sort(&mut self) {
        self.recursive_sort(0, self.nums.len() - 1);
    }

    fn recursive_sort(&mut self, left: usize, right: usize) {
        const THRESHOLD: usize = 10_000;

        if right <= left {
            return;
        }

        if right - left < THRESHOLD {
            self.nums[left..=right].sort(); // In-place sort
            return;
        }

        let mid = left + (right - left) / 2;

        // Split mutable references non-overlapping and create owned slices
        let left_part = self.nums[left..=mid].to_vec();
        let right_part = self.nums[mid + 1..=right].to_vec();

        // Spawn threads for the left and right halves
        let left_thread = thread::spawn(move || {
            let mut left_sort = ParallelMergeSort::new(left_part);
            left_sort.sort();
            left_sort.nums // Return sorted vector
        });

        let right_thread = thread::spawn(move || {
            let mut right_sort = ParallelMergeSort::new(right_part);
            right_sort.sort();
            right_sort.nums // Return sorted vector
        });

        // Wait for both threads to finish and get the sorted results
        let left_sorted = left_thread.join().unwrap();
        let right_sorted = right_thread.join().unwrap();

        // Merge the sorted halves back into the original vector
        self.merge(left, right, left_sorted, right_sorted);
    }

    fn merge(&mut self, left: usize, _right: usize, left_sorted: Vec<i32>, right_sorted: Vec<i32>) {
        let mut result = Vec::new();
        let (mut i, mut j) = (0, 0);

        while i < left_sorted.len() && j < right_sorted.len() {
            if left_sorted[i] <= right_sorted[j] {
                result.push(left_sorted[i]);
                i += 1;
            } else {
                result.push(right_sorted[j]);
                j += 1;
            }
        }

        while i < left_sorted.len() {
            result.push(left_sorted[i]);
            i += 1;
        }

        while j < right_sorted.len() {
            result.push(right_sorted[j]);
            j += 1;
        }

        // Write the merged result back to the original slice
        for (k, &value) in result.iter().enumerate() {
            self.nums[left + k] = value;
        }
    }
}
