# this is a ATM Machine Simulator
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