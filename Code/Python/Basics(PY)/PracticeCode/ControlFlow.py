'''
#This is a ATM Machine Simulator
balance = 1000

def atm():
    print("Welcome to the ATM Machine")

    pin = input("Please enter your pin: ")
    if len(pin) != 5:
        print("Invalid pin")
        return
    if pin.isdigit() == False:
        print("Invalid pin format")
        return
    if pin == "12345":
        print("Pin accepted")
        
        is_running = True
        while is_running:
            # Display menu
            print("\nPlease select an option:")
            print("1. Check Balance")
            print("2. Deposit")
            print("3. Withdraw")
            print("4. Transfer")
            print("5. Show Menu")
            print("6. Quit")
            
            # Get user choice
            choice = input("\nEnter your choice (1-6): ")
            
            # Process the choice
            if choice == "1":
                print(f"Your balance is ${balance}")
            
            elif choice == "2":
                amount = int(input("Please enter the amount you want to deposit: $"))
                balance += amount
                print(f"Your new balance is ${balance}")
            
            elif choice == "3":
                amount = int(input("Please enter the amount you want to withdraw: $"))
                if amount > balance:
                    print("Insufficient funds!")
                else:
                    balance -= amount
                    print(f"Your new balance is ${balance}")
            
            elif choice == "4":
                amount = int(input("Please enter the amount you want to transfer: $"))
                if amount > balance:
                    print("Insufficient funds!")
                else:
                    balance -= amount
                    print(f"Transfer successful. Your new balance is ${balance}")
            
            elif choice == "5":
                continue  # This will show the menu again
            
            elif choice == "6":
                print("Thank you for using the ATM Machine")
                is_running = False
            
            else:
                print("Invalid option! Please try again.")
    else:
        print("Incorrect PIN")

# Call the function to start the ATM
atm()
'''    
#This is a Temperature Converter
def converter():
    is_running = True
    while is_running:
        print("\nTemperature Converter Menu:")
        print("1. Celsius to Fahrenheit")
        print("2. Fahrenheit to Celsius")
        print("3. Celsius to Kelvin")
        print("4. Kelvin to Celsius")
        print("5. Fahrenheit to Kelvin")
        print("6. Kelvin to Fahrenheit")
        print("7. Quit")
        
        choice = input("\nPlease enter your choice (1-7): ")
        
        if choice == "1":
            celsius = float(input("Please enter the temperature in Celsius: "))
            fahrenheit = (celsius * 9/5) + 32
            print(f"{celsius} degrees Celsius is equal to {fahrenheit} degrees Fahrenheit")
            
        elif choice == "2":
            fahrenheit = float(input("Please enter the temperature in Fahrenheit: "))
            celsius = (fahrenheit - 32) * 5/9
            print(f"{fahrenheit} degrees Fahrenheit is equal to {celsius} degrees Celsius")
            
        elif choice == "3":
            celsius = float(input("Please enter the temperature in Celsius: "))
            kelvin = celsius + 273.15
            print(f"{celsius} degrees Celsius is equal to {kelvin} degrees Kelvin")
            
        elif choice == "4":
            kelvin = float(input("Please enter the temperature in Kelvin: "))
            celsius = kelvin - 273.15
            print(f"{kelvin} degrees Kelvin is equal to {celsius} degrees Celsius")
            
        elif choice == "5":
            fahrenheit = float(input("Please enter the temperature in Fahrenheit: "))
            celsius = (fahrenheit - 32) * 5/9
            kelvin = celsius + 273.15
            print(f"{fahrenheit} degrees Fahrenheit is equal to {kelvin} degrees Kelvin")
            
        elif choice == "6":
            kelvin = float(input("Please enter the temperature in Kelvin: "))
            celsius = kelvin - 273.15
            fahrenheit = (celsius * 9/5) + 32
            print(f"{kelvin} degrees Kelvin is equal to {fahrenheit} degrees Fahrenheit")
            
        elif choice == "7":
            print("Thank you for using the Temperature Converter!")
            is_running = False
            
        else:
            print("Invalid choice! Please try again.")

# Call the function to start the Temperature Converter
converter()