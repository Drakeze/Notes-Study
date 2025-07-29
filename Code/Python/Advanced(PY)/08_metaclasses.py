# Metaclasses - Classes that Create Classes

# Understanding type()
print("=== Understanding type() ===")
# TODO: Practice using type() to create classes dynamically
# Example:
# def init_method(self, value):
#     self.value = value
# 
# DynamicClass = type('DynamicClass', (), {'__init__': init_method})

# Basic metaclass
print("\n=== Basic Metaclass ===")
# TODO: Practice creating a basic metaclass
# class MyMetaclass(type):
#     def __new__(cls, name, bases, attrs):
#         print(f"Creating class {name}")
#         return super().__new__(cls, name, bases, attrs)
# 
# class MyClass(metaclass=MyMetaclass):
#     pass

# Metaclass with __init__
print("\n=== Metaclass with __init__ ===")
# TODO: Practice metaclass __init__ method
# class InitMetaclass(type):
#     def __init__(cls, name, bases, attrs):
#         print(f"Initializing class {name}")
#         super().__init__(name, bases, attrs)

# Modifying class creation
print("\n=== Modifying Class Creation ===")
# TODO: Practice modifying classes during creation
# class AutoPropertyMetaclass(type):
#     def __new__(cls, name, bases, attrs):
#         for key, value in attrs.items():
#             if key.startswith('_') and not key.startswith('__'):
#                 # Create property for private attributes
#                 attrs[key[1:]] = property(lambda self, k=key: getattr(self, k))
#         return super().__new__(cls, name, bases, attrs)

# Singleton metaclass
print("\n=== Singleton Metaclass ===")
# TODO: Practice implementing singleton pattern with metaclass
# class Singleton(type):
#     _instances = {}
#     def __call__(cls, *args, **kwargs):
#         if cls not in cls._instances:
#             cls._instances[cls] = super().__call__(*args, **kwargs)
#         return cls._instances[cls]

# Abstract base classes
print("\n=== Abstract Base Classes ===")
# TODO: Practice using abc module
# from abc import ABC, abstractmethod
# class AbstractClass(ABC):
#     @abstractmethod
#     def must_implement(self):
#         pass

if __name__ == "__main__":
    print("Practice your metaclasses here!")

