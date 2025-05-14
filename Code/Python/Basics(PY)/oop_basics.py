# Understanding Object-Oriented Programming in Python

# 1. Creating a Class
class Person:
    # Class attribute (shared by all instances)
    species = "Human"
    
    # Constructor (initialize instance attributes)
    def __init__(self, name, age):
        # Instance attributes (unique to each instance)
        self.name = name
        self.age = age
    
    # Instance method
    def introduce(self):
        return f"Hi, I'm {self.name} and I'm {self.age} years old."
    
    # Class method (works with class attributes)
    @classmethod
    def get_species(cls):
        return cls.species
    
    # Static method (doesn't need class or instance data)
    @staticmethod
    def is_adult(age):
        return age >= 18

# Example usage:
# Creating instances (objects) of the Person class
person1 = Person("Alice", 25)
person2 = Person("Bob", 30)

# Accessing instance methods and attributes
print(person1.introduce())  # Output: Hi, I'm Alice and I'm 25 years old.
print(person2.name)        # Output: Bob

# Accessing class method and static method
print(Person.get_species())  # Output: Human
print(Person.is_adult(20))   # Output: True