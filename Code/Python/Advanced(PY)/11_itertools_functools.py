# Itertools and Functools - Advanced Utilities

import itertools
import functools
from operator import add, mul

# Itertools - infinite iterators
print("=== Infinite Iterators ===")
# TODO: Practice itertools.count(), cycle(), repeat()
# counter = itertools.count(start=10, step=2)
# cycler = itertools.cycle(['A', 'B', 'C'])
# repeater = itertools.repeat('hello', 3)

# Itertools - combinatorial iterators
print("\n=== Combinatorial Iterators ===")
# TODO: Practice permutations, combinations, product
# from itertools import permutations, combinations, product
# data = ['A', 'B', 'C']
# perms = list(permutations(data, 2))
# combs = list(combinations(data, 2))
# prod = list(product(['A', 'B'], [1, 2]))

# Itertools - filtering
print("\n=== Filtering Iterators ===")
# TODO: Practice filterfalse, takewhile, dropwhile
# from itertools import filterfalse, takewhile, dropwhile
# numbers = [1, 3, 5, 2, 4, 6, 8]
# odd_only = list(filterfalse(lambda x: x % 2 == 0, numbers))
# take_odds = list(takewhile(lambda x: x % 2 == 1, numbers))
# drop_odds = list(dropwhile(lambda x: x % 2 == 1, numbers))

# Itertools - grouping
print("\n=== Grouping ===")
# TODO: Practice groupby
# from itertools import groupby
# data = [('A', 1), ('A', 2), ('B', 3), ('B', 4), ('C', 5)]
# grouped = {k: list(g) for k, g in groupby(data, key=lambda x: x[0])}

# Functools - reduce
print("\n=== Functools Reduce ===")
# TODO: Practice using reduce
# numbers = [1, 2, 3, 4, 5]
# sum_all = functools.reduce(add, numbers)
# product_all = functools.reduce(mul, numbers)

# Functools - partial
print("\n=== Functools Partial ===")
# TODO: Practice partial function application
# def multiply(x, y):
#     return x * y
# 
# double = functools.partial(multiply, 2)
# triple = functools.partial(multiply, 3)

# Functools - lru_cache
print("\n=== LRU Cache ===")
# TODO: Practice caching with lru_cache
# @functools.lru_cache(maxsize=128)
# def fibonacci(n):
#     if n < 2:
#         return n
#     return fibonacci(n-1) + fibonacci(n-2)

# Functools - singledispatch
print("\n=== Single Dispatch ===")
# TODO: Practice function overloading with singledispatch
# @functools.singledispatch
# def process(arg):
#     print(f"Processing {arg}")
# 
# @process.register(int)
# def _(arg):
#     print(f"Processing integer: {arg}")
# 
# @process.register(str)
# def _(arg):
#     print(f"Processing string: {arg}")

if __name__ == "__main__":
    print("Practice your itertools and functools here!")

