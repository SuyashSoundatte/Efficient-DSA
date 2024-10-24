// src/merge_sort.rs
pub struct MergeSort<'a> {
    nums: &'a mut Vec<i32>,
}

impl<'a> MergeSort<'a> {
    pub fn new(nums: &'a mut Vec<i32>) -> Self {
        MergeSort { nums }
    }

    pub fn sort(&mut self) {
        if self.nums.is_empty() {
            panic!("Vector is empty");
        }
        self.recursive_sort(0, self.nums.len() - 1);
    }

    fn recursive_sort(&mut self, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mid = left + (right - left) / 2;
        self.recursive_sort(left, mid);
        self.recursive_sort(mid + 1, right);

        let mut result = Vec::new();
        let (mut i, mut j) = (left, mid + 1);

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

        for (k, &value) in result.iter().enumerate() {
            self.nums[left + k] = value;
        }
    }
}
