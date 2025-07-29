# Performance Optimization in Python

import time
import cProfile
import pstats
from functools import lru_cache
from collections import defaultdict, deque
import sys

# Profiling code
print("=== Profiling Code ===")
# TODO: Practice profiling with cProfile
# def slow_function():
#     total = 0
#     for i in range(1000000):
#         total += i * i
#     return total
# 
# # Profile the function
# cProfile.run('slow_function()')

# Memory optimization
print("\n=== Memory Optimization ===")
# TODO: Practice memory-efficient coding
# Using generators instead of lists
# def memory_efficient_range(n):
#     for i in range(n):
#         yield i * i
# 
# # Using __slots__
# class OptimizedClass:
#     __slots__ = ['x', 'y', 'z']
#     def __init__(self, x, y, z):
#         self.x = x
#         self.y = y
#         self.z = z

# Caching strategies
print("\n=== Caching Strategies ===")
# TODO: Practice different caching techniques
# @lru_cache(maxsize=128)
# def expensive_function(n):
#     time.sleep(0.1)  # Simulate expensive operation
#     return n * n
# 
# # Manual caching
# cache = {}
# def manual_cache_function(n):
#     if n not in cache:
#         cache[n] = n * n
#     return cache[n]

# Efficient data structures
print("\n=== Efficient Data Structures ===")
# TODO: Practice using efficient data structures
# from collections import Counter, defaultdict, deque
# 
# # Using Counter for counting
# data = ['a', 'b', 'a', 'c', 'b', 'a']
# counter = Counter(data)
# 
# # Using defaultdict to avoid KeyError
# grouped = defaultdict(list)
# for item in data:
#     grouped[item].append(item)
# 
# # Using deque for efficient append/pop operations
# queue = deque()
# queue.appendleft(1)
# queue.append(2)

# String operations optimization
print("\n=== String Operations ===")
# TODO: Practice efficient string operations
# # Using join instead of concatenation
# parts = ['hello', 'world', 'python']
# # Slow: result = ''
# # for part in parts:
# #     result += part
# # Fast:
# result = ''.join(parts)
# 
# # Using f-strings for formatting
# name = "Python"
# version = 3.11
# message = f"Welcome to {name} {version}"

# List comprehensions vs loops
print("\n=== List Comprehensions vs Loops ===")
# TODO: Compare performance of different approaches
# import timeit
# 
# # Method 1: List comprehension
# def list_comp():
#     return [x*2 for x in range(1000)]
# 
# # Method 2: Regular loop
# def regular_loop():
#     result = []
#     for x in range(1000):
#         result.append(x*2)
#     return result
# 
# # Timing comparison
# time1 = timeit.timeit(list_comp, number=10000)
# time2 = timeit.timeit(regular_loop, number=10000)

# NumPy for numerical operations
print("\n=== NumPy for Performance ===")
# TODO: Practice using NumPy for fast numerical operations
# import numpy as np
# 
# # Pure Python (slow)
# def python_sum(data):
#     return sum(x*x for x in data)
# 
# # NumPy (fast)
# def numpy_sum(data):
#     arr = np.array(data)
#     return np.sum(arr * arr)

# Algorithm optimization
print("\n=== Algorithm Optimization ===")
# TODO: Practice choosing better algorithms
# # Binary search vs linear search
# def linear_search(arr, target):
#     for i, val in enumerate(arr):
#         if val == target:
#             return i
#     return -1
# 
# def binary_search(arr, target):
#     left, right = 0, len(arr) - 1
#     while left <= right:
#         mid = (left + right) // 2
#         if arr[mid] == target:
#             return mid
#         elif arr[mid] < target:
#             left = mid + 1
#         else:
#             right = mid - 1
#     return -1

if __name__ == "__main__":
    print("Practice your performance optimization here!")

