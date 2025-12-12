# Advent of Code 2025 in Rust

Times[^1]:

| Day | Part 1 | Part 2  |
|----:|:-------|:--------|
|  01 | 157μs  | 159μs   |
|  02 | 1.68ms | 4.88ms  |
|  03 | 159μs  | 411μs   |
|  04 | 102μs  | 1.95ms  |
|  05 | 149μs  | 16.2μs  |
|  06 | 143μs  | 48.2μs  |
|  07 | 25.8μs | 21.3μs  |
|  08 | 27.2ms | 31.5ms  |
|  09 | 91.5μs | 18.1ms  |
|  10 | 42.9ms | 36.6ms  |
|  11 | 259μs  | 430μs   |
|  12 | 190μs  | N/A[^2] |
|     |        |         |
| Max | 42.9ms | 36.6ms  |
| Avg | 831μs  | 150ms   |

Running all twelve days in order takes 224ms.

[^1]: All measurements where taken on a Ryzen 5800X @ 4.85GHz on Fedora 43 using Rust v1.92.0 stable.  
Measurement represents fastest run of about 10 total runs.
[^2]: Day 12 has no Part 2.
