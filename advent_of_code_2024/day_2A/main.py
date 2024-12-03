file = open("input.txt", 'r')

def is_sorted(row):
    if row == sorted(row):
        return True
    if row == sorted(row, reverse=True):
        return True
    return False

tot_safe = 0
for row in file:
    row = list(map(int, row.split(" ")))
    is_safe = True
    inc = False
    dec = False
    if not is_sorted(row):
        continue
    for e in range(len(row) - 1):
        a = row[e]
        b = row[e + 1]
        diff = abs(a - b)
        if diff > 3 or diff < 1:
            is_safe = False
            break
        
    if is_safe:
        tot_safe += 1

print("Safe: ", tot_safe)
