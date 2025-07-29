# Testing in Python - unittest, pytest, and more

import unittest
from unittest.mock import Mock, patch, MagicMock

# Basic unittest
print("=== Basic unittest ===")
# TODO: Practice writing unit tests
# class TestMathOperations(unittest.TestCase):
#     def test_addition(self):
#         self.assertEqual(2 + 2, 4)
#     
#     def test_division(self):
#         self.assertEqual(10 / 2, 5)
#         with self.assertRaises(ZeroDivisionError):
#             10 / 0
#     
#     def setUp(self):
#         # Setup before each test
#         self.calculator = Calculator()
#     
#     def tearDown(self):
#         # Cleanup after each test
#         pass

# Test fixtures and setup
print("\n=== Test Fixtures ===")
# TODO: Practice setUpClass, tearDownClass, setUp, tearDown
# class TestWithFixtures(unittest.TestCase):
#     @classmethod
#     def setUpClass(cls):
#         # Setup once for all tests in class
#         pass
#     
#     @classmethod
#     def tearDownClass(cls):
#         # Cleanup once after all tests
#         pass

# Mocking
print("\n=== Mocking ===")
# TODO: Practice using Mock objects
# class TestWithMocks(unittest.TestCase):
#     def test_with_mock(self):
#         mock_obj = Mock()
#         mock_obj.method.return_value = 42
#         result = mock_obj.method()
#         self.assertEqual(result, 42)
#         mock_obj.method.assert_called_once()
#     
#     @patch('module.function')
#     def test_with_patch(self, mock_function):
#         mock_function.return_value = 'mocked'
#         # Test code that uses the mocked function
#         pass

# Parametrized tests
print("\n=== Parametrized Tests ===")
# TODO: Practice parametrized tests (with pytest)
# import pytest
# @pytest.mark.parametrize("input,expected", [
#     (2, 4),
#     (3, 9),
#     (4, 16)
# ])
# def test_square(input, expected):
#     assert input ** 2 == expected

# Testing exceptions
print("\n=== Testing Exceptions ===")
# TODO: Practice testing that exceptions are raised
# def test_exception_raised(self):
#     with self.assertRaises(ValueError):
#         raise ValueError("Test error")
#     
#     with self.assertRaisesRegex(ValueError, "Test.*"):
#         raise ValueError("Test error message")

# Testing async code
print("\n=== Testing Async Code ===")
# TODO: Practice testing async functions
# import asyncio
# class TestAsync(unittest.TestCase):
#     async def test_async_function(self):
#         result = await some_async_function()
#         self.assertEqual(result, expected_value)
#     
#     def test_async_wrapper(self):
#         asyncio.run(self.test_async_function())

# Property-based testing
print("\n=== Property-based Testing ===")
# TODO: Practice property-based testing with hypothesis
# from hypothesis import given, strategies as st
# @given(st.integers())
# def test_property(x):
#     assert x + 0 == x

# Coverage and test discovery
print("\n=== Coverage and Discovery ===")
# TODO: Practice running tests with coverage
# Run: python -m coverage run -m pytest
# Run: python -m coverage report

if __name__ == "__main__":
    # Run tests
    unittest.main()

