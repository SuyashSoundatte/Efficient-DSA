#ifndef THREADMERGESORT_H
#define THREADMERGESORT_H

#include <vector>
#include <thread>
#include <mutex>
#include <iostream>
#include <algorithm>

class ThreadMergeSort{
    private:
        std::vector<int> *nums;

    public:
        ThreadMergeSort(std::vector<int> *nums);
        ~ThreadMergeSort();
        void sort();
        void recursiveSort(int left, int right);
};

#endif