#calculator app
import math 

num1_str , operator, num2_str = expression.split()

num1 = float(num1_str)
num2 = float(num1_str)

#operator factors

if operator =="+":
    result = num1 + num2
elseif operator == "-":
    result = num1 - num2
elseif operator == "*":
    result = num1 * num2
elseif operator == "/":
    result = num1 / num2
elseif operator == "//":
    result = num1 // num2
elseif operator == "%":
    result = num1 % num2
elseif operator == "**"
    result = num1 ** num2
 if num2 != 0:
        result = num1 / num2
    else:
        result = "Error: Division by zero"
else:
    result = "Invalid operator"

# Show the result
print("Result:", result)


