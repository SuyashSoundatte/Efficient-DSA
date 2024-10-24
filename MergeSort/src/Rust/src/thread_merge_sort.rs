use std::thread;

pub struct ParallelMergeSort<'a> {
    nums: &'a mut Vec<i32>,
}

impl<'a> ParallelMergeSort<'a> {
    pub fn new(nums: &'a mut Vec<i32>) -> Self {
        Self { nums }
    }

    pub fn sort(&mut self) {
        self.recursive_sort(0, self.nums.len() - 1);
    }

    fn recursive_sort(&mut self, left: usize, right: usize) {
        const THRESHOLD: usize = 10_000;

        if right - left < THRESHOLD {
            self.nums[left..=right].sort();
            return;
        }

        if left >= right {
            return;
        }

        let mid = left + (right - left) / 2;

        // Clone for parallel threads
        let mut nums_clone_1 = self.nums.clone(); // Declare this as mutable
        let mut nums_clone_2 = self.nums.clone(); // Declare this as mutable

        // Spawn the first thread for the left half
        let thread_1 = thread::spawn(move || {
            let mut sort = ParallelMergeSort::new(&mut nums_clone_1);
            sort.recursive_sort(left, mid);
        });

        // Spawn the second thread for the right half
        let thread_2 = thread::spawn(move || {
            let mut sort = ParallelMergeSort::new(&mut nums_clone_2);
            sort.recursive_sort(mid + 1, right);
        });

        thread_1.join().unwrap();
        thread_2.join().unwrap();

        let mut result = Vec::new();
        let mut i = left;
        let mut j = mid + 1;

        // Merge sorted halves
        while i <= mid && j <= right {
            if self.nums[i] <= self.nums[j] {
                result.push(self.nums[i]);
                i += 1;
            } else {
                result.push(self.nums[j]);
                j += 1;
            }
        }

        while i <= mid {
            result.push(self.nums[i]);
            i += 1;
        }

        while j <= right {
            result.push(self.nums[j]);
            j += 1;
        }

        // Write the merged result back to the original slice
        for (k, &value) in result.iter().enumerate() {
            self.nums[left + k] = value;
        }
    }
}
