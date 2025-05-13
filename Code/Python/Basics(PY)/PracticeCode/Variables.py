#this is a basic python program
'''print ("Sign In")
name = input("Enter your name: ")
password = input("Enter your password: ")
# Condition for when the password is correct
if password == password:
    print("Welcome, " + name + "!")
else:
    print("Incorrect password.")

# Condition for when the age is correct
age = int(input("Enter your age: "))
if age >= 18:
    print("Welcome to the club, " + name + "!")
else:
    print("Sorry, you are not old enough to enter" + name + ".")'''

#This is a more advanced sign in program
import getpass
print("Sign In")

# First check age requirement
try:
    age = int(input("Enter your age: "))
    if age < 18:
        print("Sorry, you must be 18 or older to create an account.")
        exit()
except ValueError:
    print("Invalid input. Please enter a number.")
    exit()

# Get name (with validation)
while True:
    name = input("Enter your name: ").strip()
    if name:  # Check if name is not empty
        break
    print("Name cannot be empty.")

# Set up password (with validation)
while True:
    password = getpass.getpass("Create a password: ")
    if password:  # Check if password is not empty
        confirm_password = getpass.getpass("Confirm password: ")
        if password == confirm_password:
            break
        print("Passwords do not match. Please try again.")
    else:
        print("Password cannot be empty.")

# retry limit 
max_attempts = 3
attempts = 0
while attempts < max_attempts:
    password = getpass.getpass("Enter your password: ")
    if password == password:
        print(f"Welcome, {name}! Your account has been created successfully.")
        break
    else:
        attempts += 1
        remaining_attempts = max_attempts - attempts




    