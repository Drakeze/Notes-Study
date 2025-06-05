# Advanced Error Handling and Exceptions

# Basic exception handling
print("=== Basic Exception Handling ===")
# TODO: Practice try, except, else, finally
# try:
#     result = 10 / 0
# except ZeroDivisionError:
#     print("Cannot divide by zero")
# else:
#     print("Division successful")
# finally:
#     print("Cleanup code here")

# Multiple exception types
print("\n=== Multiple Exception Types ===")
# TODO: Practice handling multiple exception types
# try:
#     # Some code that might raise different exceptions
#     pass
# except (ValueError, TypeError) as e:
#     print(f"Value or Type error: {e}")
# except Exception as e:
#     print(f"Other error: {e}")

# Custom exceptions
print("\n=== Custom Exceptions ===")
# TODO: Practice creating custom exception classes
# class CustomError(Exception):
#     def __init__(self, message, error_code=None):
#         super().__init__(message)
#         self.error_code = error_code
# 
# class ValidationError(CustomError):
#     pass

# Exception chaining
print("\n=== Exception Chaining ===")
# TODO: Practice exception chaining with 'raise from'
# try:
#     # Some operation
#     1 / 0
# except ZeroDivisionError as e:
#     raise ValueError("Invalid calculation") from e

# Context managers for exception handling
print("\n=== Context Managers for Exceptions ===")
# TODO: Practice using context managers for cleanup
# from contextlib import contextmanager
# @contextmanager
# def error_handler():
#     try:
#         yield
#     except Exception as e:
#         print(f"Handled error: {e}")
#         # Log error, send notification, etc.

# Suppressing exceptions
print("\n=== Suppressing Exceptions ===")
# TODO: Practice suppressing exceptions with contextlib
# from contextlib import suppress
# with suppress(FileNotFoundError):
#     with open('nonexistent.txt') as f:
#         content = f.read()

# Exception groups (Python 3.11+)
print("\n=== Exception Groups ===")
# TODO: Practice exception groups (if using Python 3.11+)
# try:
#     raise ExceptionGroup("Multiple errors", [
#         ValueError("Invalid value"),
#         TypeError("Wrong type")
#     ])
# except* ValueError as eg:
#     print(f"Caught ValueError group: {eg}")
# except* TypeError as eg:
#     print(f"Caught TypeError group: {eg}")

# Logging exceptions
print("\n=== Logging Exceptions ===")
# TODO: Practice logging exceptions properly
# import logging
# logging.basicConfig(level=logging.INFO)
# logger = logging.getLogger(__name__)
# 
# try:
#     # Some operation
#     pass
# except Exception:
#     logger.exception("An error occurred")

if __name__ == "__main__":
    print("Practice your error handling here!")

