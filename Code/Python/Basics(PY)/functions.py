# Basic Function
def greet():
    print("Hello, World!")

# Function with Parameters
def greet_person(name):
    print(f"Hello, {name}!")

# Function with Default Parameter
def greet_with_title(name, title="Mr."):
    print(f"Hello, {title} {name}!")

# Function with Multiple Parameters
def calculate_sum(a, b):
    return a + b

# Function with Multiple Return Values
def calculate_math(a, b):
    sum_result = a + b
    diff_result = a - b
    return sum_result, diff_result

# Function with *args (Variable Number of Arguments)
def sum_all(*numbers):
    total = 0
    for num in numbers:
        total += num
    return total

# Function with **kwargs (Keyword Arguments)
def print_info(**kwargs):
    for key, value in kwargs.items():
        print(f"{key}: {value}")

# Lambda Function (Anonymous Function)
square = lambda x: x * x

# Function with Type Hints (Python 3.5+)
def multiply(x: int, y: int) -> int:
    return x * y

# Example usage:
if __name__ == "__main__":
    # Basic function call
    greet()  # Output: Hello, World!
    
    # Function with parameter
    greet_person("Alice")  # Output: Hello, Alice!
    
    # Function with default parameter
    greet_with_title("Bob")  # Output: Hello, Mr. Bob!
    greet_with_title("Alice", "Dr.")  # Output: Hello, Dr. Alice!
    
    # Function with return value
    result = calculate_sum(5, 3)
    print(f"Sum: {result}")  # Output: Sum: 8
    
    # Multiple return values
    sum_val, diff_val = calculate_math(10, 4)
    print(f"Sum: {sum_val}, Difference: {diff_val}")  # Output: Sum: 14, Difference: 6
    
    # Variable number of arguments
    print(sum_all(1, 2, 3, 4, 5))  # Output: 15
    
    # Keyword arguments
    print_info(name="John", age=30, city="New York")
    
    # Lambda function
    print(f"Square of 5: {square(5)}")  # Output: Square of 5: 25
    
    # Function with type hints
    print(f"Product: {multiply(6, 7)}")  # Output: Product: 42