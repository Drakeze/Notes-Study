'''
#basic calculator
# setting the variables to none so that we can use them in the functions
num1 = None
num2 = None
operator = None
def get_input():
    global num1, num2, operator
    num1 = float(input("Enter the first number: "))
    num2 = float(input("Enter the second number: "))
    operator = input("Enter the operator (+, -, *, /): ")
#a if statement case to check the operator and return the result
def calculate():
    if operator == "+":
        return num1 + num2
    elif operator == "-":
        return num1 - num2
    elif operator == "*":
        return num1 * num2
    elif operator == "/":
        if num2 != 0:
            return num1 / num2
        else:
            return "Error: Division by zero"
    else:
        return "Error: Invalid operator"

# Main execution block
if __name__ == "__main__":
    try:
        get_input()  # Get user input
        result = calculate()  # Perform calculation
        print(f"Result: {result}")  # Display result
    except ValueError:
        print("Error: Please enter valid numbers")
    except Exception as e:
        print(f"An error occurred: {e}")
'''
# This is a Password validator
# Helper functions for password validation
def has_uppercase(password):
    for char in password:
        if char.isupper():
            return True
    return False

def has_lowercase(password):
    for char in password:
        if char.islower():
            return True
    return False

def has_numbers(password):
    for char in password:
        if char.isdigit():
            return True
    return False

def has_special_chars(password):
    special_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?"
    for char in password:
        if char in special_chars:
            return True
    return False

# Main password validator function
def __Pass__():
    password = input("Enter your password: ")
    
    # Length validation
    if len(password) < 8:
        return "Password is too short"
    if len(password) > 32:
        return "Password is too long"
        
    # Character validations
    if not has_uppercase(password):
        return "Password must contain at least one uppercase letter"
    if not has_lowercase(password):
        return "Password must contain at least one lowercase letter"
    if not has_numbers(password):
        return "Password must contain at least one digit"
    if not has_special_chars(password):
        return "Password must contain at least one special character"
    
    # If all checks pass
    return "Password is valid!"

# Test the password validator
if __name__ == "__main__":
    result = __Pass__()
    print(result)