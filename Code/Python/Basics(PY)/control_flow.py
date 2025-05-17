# Basic Control Flow Examples in Python

# 1. If-Elif-Else Statements
def check_number(num):
    if num > 0:
        print(f"{num} is positive")
    elif num < 0:
        print(f"{num} is negative")
    else:
        print("The number is zero")

# 2. For Loops
def demonstrate_for_loop():
    # Simple range loop
    for i in range(5):
        print(f"Count: {i}")
    
    # Loop through list
    fruits = ["apple", "banana", "cherry"]
    for fruit in fruits:
        print(f"Fruit: {fruit}")

# 3. While Loops
def countdown(start):
    while start > 0:
        print(f"Countdown: {start}")
        start -= 1
    print("Blast off!")

# 4. Break and Continue
def break_continue_example():
    # Break example
    for i in range(10):
        if i == 5:
            break
        print(f"Break example: {i}")
    
    # Continue example
    for i in range(5):
        if i == 2:
            continue
        print(f"Continue example: {i}")

# 5. Try-Except (Exception Handling)
def exception_handling():
    try:
        x = int(input("Enter a number: "))
        result = 10 / x
        print(f"Result: {result}")
    except ValueError:
        print("Please enter a valid number")
    except ZeroDivisionError:
        print("Cannot divide by zero")
    finally:
        print("This always executes")

# 6. Match Case (Python 3.10+)
def match_case_example(status):
    match status:
        case "200":
            print("OK")
        case "404":
            print("Not Found")
        case "500":
            print("Server Error")
        case _:
            print("Unknown Status")

# Example usage
if __name__ == "__main__":
    print("Control Flow Examples:")
    check_number(5)
    demonstrate_for_loop()
    countdown(3)
    break_continue_example()
    exception_handling()
    match_case_example("404")