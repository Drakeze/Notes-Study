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
    