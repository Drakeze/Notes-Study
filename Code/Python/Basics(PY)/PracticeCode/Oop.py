#Practice Code for OOP
class BankAccount:
    accounts = {}
# This is the constructor for the BankAccount class
# sing the __init__ method to initialize the attributes of the class
# self: refers to the instance of the class being created
# account_number: a unique identifier for the account
# initial_balance: the initial balance of the account (default is 0)
def __init__(self, account_number, initial_balance=0, account_type="Active"):
    self.account_number = account_number
    self.balance = initial_balance
    account_number = len(BankAccount.accounts) + 1001
    BankAccount.accounts[account_number] = self
# This is for account information
def __str__(self):
    account_info = {
        "Account Number": self.account_number,
        "Account Type": self.account_type,
        "Balance": f"${self.balance}",
        "Status": "Active" if self.account_number in BankAccount.accounts else "Inactive"
    }
    return str (account_info)
#this is for changing account type active or inactive
def change_account_type( self, change_type):
    if change_type.lower() == "active":
        self.account_type = "Active"
    elif change_type.lower() == "inactive":
        self.account_type = "Inactive"
    else:
        print("Invalid account type. Please choose 'active' or 'inactive'.")
#This is for multiple accounts
MAX_ACCOUNTS = 3
#This is for creating accounts
def create_accounts(num_accounts):
    accounts = []
    if num_accounts > 0 and num_accounts <= MAX_ACCOUNTS:
        for i in range(num_accounts):
            account_number = len(BankAccount.accounts) + 1001
            account = BankAccount(account_number)
            accounts.append(account)
    else:
        print(f"Invalid number of accounts. Please create between 1 and {MAX_ACCOUNTS} accounts.")
    return []

# this is now how to show a balance change 
def deposit(self, amount):
    if amount > 0:
        self.balance += amount
        print(f"Deposited ${amount}. New balance: ${self.balance}")
    else:
        print("Invalid deposit amount.")
# This is for the transfer method
def transfer(self,target_account, amount):
    if amount > 0 and amount <= self.balance:
        self.balance -= amount
        target_account.balance += amount
        print(f"Transferred ${amount} to Account {target_account.account_number}. New balance: ${self.balance}")
    else:
        print("Invalid transfer amount or insufficient funds.")
# this is for the withdraw method
def withdraw(self, amount):
    if 0 < amount <= self.balance:
        self.balance -= amount
        print(f"Withdrew ${amount}. New balance: ${self.balance}")
    else:
        print("Invalid withdrawal amount or insufficient funds.")
#This is for the check balance method
def check_balance(self):
    try:
        print(f"Account Balance for {self.account_number}: ${self.balance}")
    except AttributeError:
        print("Error: Account not properly initialized.")
