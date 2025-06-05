# Descriptors - Attribute Access Control

# Basic descriptor
print("=== Basic Descriptor ===")
# TODO: Practice creating a basic descriptor
# class MyDescriptor:
#     def __get__(self, obj, objtype=None):
#         print("Getting value")
#         return self.value
#     
#     def __set__(self, obj, value):
#         print(f"Setting value to {value}")
#         self.value = value
#     
#     def __delete__(self, obj):
#         print("Deleting value")
#         del self.value

# Data vs Non-data descriptors
print("\n=== Data vs Non-data Descriptors ===")
# TODO: Practice the difference between data and non-data descriptors
# class DataDescriptor:
#     def __get__(self, obj, objtype=None):
#         return "data descriptor"
#     
#     def __set__(self, obj, value):
#         pass
# 
# class NonDataDescriptor:
#     def __get__(self, obj, objtype=None):
#         return "non-data descriptor"

# Property descriptor
print("\n=== Property Descriptor ===")
# TODO: Practice using property as a descriptor
# class Circle:
#     def __init__(self, radius):
#         self._radius = radius
#     
#     @property
#     def radius(self):
#         return self._radius
#     
#     @radius.setter
#     def radius(self, value):
#         if value < 0:
#             raise ValueError("Radius cannot be negative")
#         self._radius = value

# Custom validation descriptor
print("\n=== Custom Validation Descriptor ===")
# TODO: Practice creating validation descriptors
# class ValidatedAttribute:
#     def __init__(self, validator):
#         self.validator = validator
#         self.name = None
#     
#     def __set_name__(self, owner, name):
#         self.name = f"_{name}"
#     
#     def __get__(self, obj, objtype=None):
#         return getattr(obj, self.name)
#     
#     def __set__(self, obj, value):
#         if self.validator(value):
#             setattr(obj, self.name, value)
#         else:
#             raise ValueError(f"Invalid value: {value}")

# Method descriptors
print("\n=== Method Descriptors ===")
# TODO: Practice understanding how methods work as descriptors
# Functions are descriptors that return bound methods when accessed

# Descriptor with __set_name__
print("\n=== Descriptor with __set_name__ ===")
# TODO: Practice using __set_name__ for automatic naming

if __name__ == "__main__":
    print("Practice your descriptors here!")

