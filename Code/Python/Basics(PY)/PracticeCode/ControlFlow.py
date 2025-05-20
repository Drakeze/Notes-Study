#This is a ATM Machine Simulator
balance = 1000
def atm():
    print("Welcome to the ATM Machine")

    pin =input("Please enter your pin")
    if len(pin) != 5:
        print("Invalid pin")
        return
    if pin.isdigit() == False:
        print("Invalid pin format")
        return
    if pin == "12345":
        print("Pin accepted")
        print("Please select an option")
# The options for the user
check_balance = 1
deposit = 2
withdraw = 3
transfer = 4
quit = 5
if check_balance == 1:
    print(f"Your balance is {balance}")
if deposit == 2:
    amount = int(input("Please enter the amount you want to deposit"))
    balance += amount
    print(f"Your new balance is {balance}")
if withdraw == 3:
    amount = int(input("Please enter the amount you want to withdraw"))
    balance -= amount
    print(f"Your new balance is {balance}")
if transfer == 4:
    amount = int(input("Please enter the amount you want to transfer"))
    balance -= amount
    print(f"Your new balance is {balance}")
if quit == 5:
    print("Thank you for using the ATM Machine")
    return