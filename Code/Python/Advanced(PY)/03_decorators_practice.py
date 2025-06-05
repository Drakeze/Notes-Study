# Decorators - Advanced Function Modification

# Basic decorator
print("=== Basic Decorator ===")
# TODO: Practice creating a simple decorator
# def my_decorator(func):
#     def wrapper():
#         print("Something before the function")
#         func()
#         print("Something after the function")
#     return wrapper

# Decorator with arguments
print("\n=== Decorator with Arguments ===")
# TODO: Practice decorators that accept function arguments
# def args_decorator(func):
#     def wrapper(*args, **kwargs):
#         print(f"Arguments: {args}, Keyword arguments: {kwargs}")
#         return func(*args, **kwargs)
#     return wrapper

# Multiple decorators
print("\n=== Multiple Decorators ===")
# TODO: Practice stacking multiple decorators

# Class-based decorators
print("\n=== Class-based Decorators ===")
# TODO: Practice creating decorators using classes
# class MyDecorator:
#     def __init__(self, func):
#         self.func = func
#     def __call__(self, *args, **kwargs):
#         # Do something before
#         result = self.func(*args, **kwargs)
#         # Do something after
#         return result

# Parameterized decorators
print("\n=== Parameterized Decorators ===")
# TODO: Practice decorators that take parameters
# def repeat(times):
#     def decorator(func):
#         def wrapper(*args, **kwargs):
#             for _ in range(times):
#                 result = func(*args, **kwargs)
#             return result
#         return wrapper
#     return decorator

# Built-in decorators
print("\n=== Built-in Decorators ===")
# TODO: Practice @property, @staticmethod, @classmethod

if __name__ == "__main__":
    print("Practice your decorators here!")

