def extract_calibration_value(line):
    # Extract the first and last digits from the line
    first_digit = int(line[0])
    last_digit = int(line[-1])
    
    # Combine to form a two-digit number
    calibration_value = int(str(first_digit) + str(last_digit))
    
    return calibration_value

def sum_calibration_values(calibration_document):
    # Split the document into lines
    lines = calibration_document.split('\n')
    
    # Initialize the sum
    total_sum = 0
    
    # Iterate through each line and add the calibration value to the sum
    for line in lines:
        if line:
            calibration_value = extract_calibration_value(line)
            total_sum += calibration_value
    
    return total_sum

# Example calibration document
calibration_document = """1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"""

# Calculate and print the sum of calibration values
result = sum_calibration_values(calibration_document)
print("The sum of all calibration values is:", result)

