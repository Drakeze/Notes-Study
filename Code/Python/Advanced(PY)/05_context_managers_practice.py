# Context Managers - Resource Management

# Basic with statement
print("=== Basic with Statement ===")
# TODO: Practice using with statement for file handling
# with open('file.txt', 'r') as f:
#     content = f.read()

# Custom context manager with class
print("\n=== Custom Context Manager (Class) ===")
# TODO: Practice creating context managers using __enter__ and __exit__
# class MyContextManager:
#     def __enter__(self):
#         print("Entering context")
#         return self
#     
#     def __exit__(self, exc_type, exc_val, exc_tb):
#         print("Exiting context")
#         return False

# Context manager with contextlib
print("\n=== Context Manager with contextlib ===")
# from contextlib import contextmanager
# TODO: Practice using @contextmanager decorator
# @contextmanager
# def my_context():
#     print("Entering")
#     try:
#         yield "resource"
#     finally:
#         print("Exiting")

# Multiple context managers
print("\n=== Multiple Context Managers ===")
# TODO: Practice using multiple context managers
# with open('file1.txt') as f1, open('file2.txt') as f2:
#     # Do something with both files
#     pass

# Exception handling in context managers
print("\n=== Exception Handling ===")
# TODO: Practice handling exceptions in context managers

# Nested context managers
print("\n=== Nested Context Managers ===")
# TODO: Practice nesting context managers

if __name__ == "__main__":
    print("Practice your context managers here!")

