def is_valid_sequence(levels):
    if len(levels) <= 1:
        return True
    
    increasing = True
    decreasing = True
    
    for i in range(len(levels) - 1):
        a = levels[i]
        b = levels[i + 1]
        diff = abs(a - b)
        
        if diff > 3 or diff < 1:
            return False
            
        if a > b:
            increasing = False
        else:
            decreasing = False
            
        if not increasing and not decreasing:
            return False
            
    return True

def solve_part2():
    total_safe = 0
    
    with open("input.txt", 'r') as file:
        for line in file:
            
            sequence = list(map(int, line.strip().split()))
            
            # Initial check, for part 1
            if is_valid_sequence(sequence):
                total_safe += 1
                continue
                
            # Brute force method for removing each element and checking them
            for i in range(len(sequence)):
                # Create new sequence without current element
                test_sequence = sequence[:i] + sequence[i+1:]
                
                if is_valid_sequence(test_sequence):
                    total_safe += 1
                    break
                    
    return total_safe

print(f"Total safe sequences: {solve_part2()}")
