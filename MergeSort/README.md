# 🚀 **Efficient-DSA**: Accelerating Algorithms with C++ and Rust 🚀

Welcome to the **Efficient-DSA** repository! This project aims to make traditional Data Structures and Algorithms (DSA) more efficient by leveraging modern multithreading, concurrency, and performance-boosting techniques using C++ and Rust. 🌟

## 🧑‍💻 Key Concepts

### 🧵 Threads

A **thread** is the smallest unit of a process that can be scheduled by the operating system. Threads within a process share resources, such as memory, and work together to improve the performance of CPU-bound tasks.

- **C++**: You can create and manage threads using `std::thread` or lower-level libraries like `pthread`.
- **Rust**: The `std::thread` module offers powerful yet safe multithreading capabilities.

```
+-------------+
|    Task     |   
+------+------+
       |
+------v------+
|   Thread 1  |  <== Multiple threads can execute concurrently
+------+------+
       |
+------v------+
|   Thread 2  |
+-------------+
```

### ⚔️ Parallelism vs Concurrency

- **Parallelism**: Executing tasks **simultaneously**, usually across multiple CPU cores. 🔀
- **Concurrency**: Multiple tasks make progress **at the same time**, often interleaving execution rather than running simultaneously. 🌀

Both C++ and Rust allow for parallelism and concurrency to speed up algorithms like sorting, searching, and graph traversals.

### 📏 Threshold in Sorting Algorithms

A **threshold** is used in sorting algorithms like merge sort to determine when to switch between serial and parallel execution. Below a certain threshold, it’s more efficient to perform operations sequentially, as creating and managing threads can become an overhead.

#### 🧮 How to Compute a Threshold

- Experiment with different data sizes and profile the performance.
- **Optimal threshold**: Typically found by balancing the overhead of creating threads with the benefit of parallel execution.

```cpp
if (data_size < THRESHOLD) {
    // Perform sequential merge sort
} else {
    // Perform parallel merge sort
}
```

---

## 🏗️ Project Structure

Here's a quick overview of the project structure to help you navigate:

```plaintext
📂 efficient-dsa
├── src/
│   ├── cpp/                # C++ implementations
│   ├── rust/               # Rust implementations
│   └── benchmarks/         # Performance benchmarks
├── README.md               # Project description and instructions
├── CONTRIBUTING.md         # Contribution guidelines
```

## To run the project

### C++

```bash
g++ -o mergesort main.cpp src/mergeSort.cpp src/threadMergeSort.cpp -pthread  
```

### Rust

```bash
cargo build --release
cargo run --release
```

## Warning

**Before running the code, please be aware of the following requirements:**

- This implementation of the MergeSort algorithm, especially the parallel version, is optimized for multi-core processors. If your machine has **less than 8 CPU cores** or **less than 8 GB of RAM**, you may experience significant slowdowns or the application may hang during execution.
- For best performance and to avoid potential issues, it is recommended to run this code on a machine with **at least 8 CPU cores** and **8 GB of RAM**.
- If you are running this on a lower-spec machine, consider reducing the array size (e.g., using a smaller constant for `SIZE`) to prevent excessive resource consumption.
