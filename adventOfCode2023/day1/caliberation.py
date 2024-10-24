file = open("input.txt", 'r')


sum = 0
for line in file:
    for letter in line:
        if letter.isnumeric():
            x = int(letter)
            y = int(letter)
            break
    for letter in line:
        if letter.isnumeric():
            y = int(letter)

    sum += (x*10) + y
print(sum)
