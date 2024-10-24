#include "src/mergeSort.hpp"
#include "src/threadMergeSort.hpp"
#include <iostream>
#include <chrono>
#include <cstdlib>
#include <algorithm>

int main(int argc, char *argv[]){
    // warning if your machine is having only 8 cores or less than 8 then it will takes lot of time or it might hang your machine so just reduce the array element size
    const int SIZE = 100000000;

    std::vector<int>nums(SIZE);
    std::vector<int>nums1(SIZE);

    for(int i=0;i<SIZE; i++){
        nums[i] = rand()%100000000;
        nums1[i] = rand()%100000000;
    }

    MergeSort* mergesort = new MergeSort(&nums);

    auto start = std::chrono::high_resolution_clock::now();
    mergesort->sort();
    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> mergesortDuration = end-start;
    std::cout<<"MergeSort algorithm time taken: "<<mergesortDuration.count()<<" seconds"<<std::endl;

    delete mergesort;


    ThreadMergeSort* threadMergesort = new ThreadMergeSort(&nums1);

    start = std::chrono::high_resolution_clock::now();
    threadMergesort->sort();
    end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> threadMergesortDuration = end-start;
    std::cout<<"Parallel MergeSort algorithm time taken: "<<threadMergesortDuration.count()<<" seconds"<<std::endl;

    delete threadMergesort;
}