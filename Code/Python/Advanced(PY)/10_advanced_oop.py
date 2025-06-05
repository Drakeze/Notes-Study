# Advanced Object-Oriented Programming

# Multiple inheritance and MRO
print("=== Multiple Inheritance and MRO ===")
# TODO: Practice multiple inheritance and method resolution order
# class A:
#     def method(self):
#         print("A")
# 
# class B(A):
#     def method(self):
#         print("B")
#         super().method()
# 
# class C(A):
#     def method(self):
#         print("C")
#         super().method()
# 
# class D(B, C):
#     def method(self):
#         print("D")
#         super().method()
# 
# # Check MRO: D.__mro__

# Abstract base classes
print("\n=== Abstract Base Classes ===")
# TODO: Practice creating abstract base classes
# from abc import ABC, abstractmethod
# class Shape(ABC):
#     @abstractmethod
#     def area(self):
#         pass
#     
#     @abstractmethod
#     def perimeter(self):
#         pass

# Class and static methods
print("\n=== Class and Static Methods ===")
# TODO: Practice @classmethod and @staticmethod
# class MyClass:
#     class_var = "class variable"
#     
#     @classmethod
#     def class_method(cls):
#         return f"Called from {cls.__name__}"
#     
#     @staticmethod
#     def static_method():
#         return "This is a static method"

# Properties and computed attributes
print("\n=== Properties and Computed Attributes ===")
# TODO: Practice creating properties
# class Rectangle:
#     def __init__(self, width, height):
#         self._width = width
#         self._height = height
#     
#     @property
#     def area(self):
#         return self._width * self._height
#     
#     @property
#     def width(self):
#         return self._width
#     
#     @width.setter
#     def width(self, value):
#         if value <= 0:
#             raise ValueError("Width must be positive")
#         self._width = value

# Slots for memory optimization
print("\n=== Slots ===")
# TODO: Practice using __slots__
# class Point:
#     __slots__ = ['x', 'y']
#     
#     def __init__(self, x, y):
#         self.x = x
#         self.y = y

# Magic methods
print("\n=== Magic Methods ===")
# TODO: Practice implementing magic methods
# class Vector:
#     def __init__(self, x, y):
#         self.x = x
#         self.y = y
#     
#     def __add__(self, other):
#         return Vector(self.x + other.x, self.y + other.y)
#     
#     def __str__(self):
#         return f"Vector({self.x}, {self.y})"
#     
#     def __eq__(self, other):
#         return self.x == other.x and self.y == other.y

if __name__ == "__main__":
    print("Practice your advanced OOP here!")

