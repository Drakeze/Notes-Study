# Design Patterns in Python

# Singleton Pattern
print("=== Singleton Pattern ===")
# TODO: Practice implementing Singleton pattern
# class Singleton:
#     _instance = None
#     
#     def __new__(cls):
#         if cls._instance is None:
#             cls._instance = super().__new__(cls)
#         return cls._instance
# 
# # Alternative using metaclass
# class SingletonMeta(type):
#     _instances = {}
#     def __call__(cls, *args, **kwargs):
#         if cls not in cls._instances:
#             cls._instances[cls] = super().__call__(*args, **kwargs)
#         return cls._instances[cls]

# Factory Pattern
print("\n=== Factory Pattern ===")
# TODO: Practice Factory pattern
# class Animal:
#     def speak(self):
#         pass
# 
# class Dog(Animal):
#     def speak(self):
#         return "Woof!"
# 
# class Cat(Animal):
#     def speak(self):
#         return "Meow!"
# 
# class AnimalFactory:
#     @staticmethod
#     def create_animal(animal_type):
#         if animal_type == "dog":
#             return Dog()
#         elif animal_type == "cat":
#             return Cat()
#         else:
#             raise ValueError("Unknown animal type")

# Observer Pattern
print("\n=== Observer Pattern ===")
# TODO: Practice Observer pattern
# class Subject:
#     def __init__(self):
#         self._observers = []
#     
#     def attach(self, observer):
#         self._observers.append(observer)
#     
#     def detach(self, observer):
#         self._observers.remove(observer)
#     
#     def notify(self, message):
#         for observer in self._observers:
#             observer.update(message)
# 
# class Observer:
#     def __init__(self, name):
#         self.name = name
#     
#     def update(self, message):
#         print(f"{self.name} received: {message}")

# Strategy Pattern
print("\n=== Strategy Pattern ===")
# TODO: Practice Strategy pattern
# class SortStrategy:
#     def sort(self, data):
#         pass
# 
# class BubbleSort(SortStrategy):
#     def sort(self, data):
#         # Implement bubble sort
#         return sorted(data)  # Simplified
# 
# class QuickSort(SortStrategy):
#     def sort(self, data):
#         # Implement quick sort
#         return sorted(data)  # Simplified
# 
# class Sorter:
#     def __init__(self, strategy):
#         self.strategy = strategy
#     
#     def sort(self, data):
#         return self.strategy.sort(data)

# Decorator Pattern
print("\n=== Decorator Pattern ===")
# TODO: Practice Decorator pattern (not Python decorators)
# class Component:
#     def operation(self):
#         pass
# 
# class ConcreteComponent(Component):
#     def operation(self):
#         return "Basic operation"
# 
# class Decorator(Component):
#     def __init__(self, component):
#         self._component = component
#     
#     def operation(self):
#         return self._component.operation()
# 
# class ConcreteDecorator(Decorator):
#     def operation(self):
#         return f"Enhanced {super().operation()}"

# Command Pattern
print("\n=== Command Pattern ===")
# TODO: Practice Command pattern
# class Command:
#     def execute(self):
#         pass
#     
#     def undo(self):
#         pass
# 
# class Light:
#     def __init__(self):
#         self.is_on = False
#     
#     def turn_on(self):
#         self.is_on = True
#         print("Light is on")
#     
#     def turn_off(self):
#         self.is_on = False
#         print("Light is off")
# 
# class LightOnCommand(Command):
#     def __init__(self, light):
#         self.light = light
#     
#     def execute(self):
#         self.light.turn_on()
#     
#     def undo(self):
#         self.light.turn_off()

# Builder Pattern
print("\n=== Builder Pattern ===")
# TODO: Practice Builder pattern
# class House:
#     def __init__(self):
#         self.walls = None
#         self.roof = None
#         self.windows = None
#     
#     def __str__(self):
#         return f"House with {self.walls}, {self.roof}, {self.windows}"
# 
# class HouseBuilder:
#     def __init__(self):
#         self.house = House()
#     
#     def build_walls(self, walls):
#         self.house.walls = walls
#         return self
#     
#     def build_roof(self, roof):
#         self.house.roof = roof
#         return self
#     
#     def build_windows(self, windows):
#         self.house.windows = windows
#         return self
#     
#     def get_house(self):
#         return self.house

if __name__ == "__main__":
    print("Practice your design patterns here!")

