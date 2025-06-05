# Generators - Memory Efficient Iterators

# Basic generator function
print("=== Basic Generator Function ===")
# TODO: Practice creating generator functions with yield
# def simple_generator():
#     yield 1
#     yield 2
#     yield 3

# Generator expressions
print("\n=== Generator Expressions ===")
# TODO: Practice generator expressions
# Example: squares_gen = (x**2 for x in range(10))

# Infinite generators
print("\n=== Infinite Generators ===")
# TODO: Practice creating infinite generators
# def fibonacci():
#     a, b = 0, 1
#     while True:
#         yield a
#         a, b = b, a + b

# Generator with send()
print("\n=== Generator with send() ===")
# TODO: Practice two-way communication with generators
# def echo_generator():
#     while True:
#         received = yield
#         print(f"Received: {received}")

# Generator pipeline
print("\n=== Generator Pipeline ===")
# TODO: Practice chaining generators together
# def numbers():
#     for i in range(10):
#         yield i
# 
# def squares(nums):
#     for num in nums:
#         yield num ** 2
# 
# def evens(nums):
#     for num in nums:
#         if num % 2 == 0:
#             yield num

# yield from
print("\n=== yield from ===")
# TODO: Practice delegating to sub-generators

if __name__ == "__main__":
    print("Practice your generators here!")

