import re

def parse_memory(memory):
    mul_regex = re.compile(r'mul\(\d+,\d+\)')
    do_regex = re.compile(r'do\(\)')
    dont_regex = re.compile(r"don't\(\)")
    
    mul_instructions = mul_regex.findall(memory)
    do_instructions = do_regex.findall(memory)
    dont_instructions = dont_regex.findall(memory)
    
    enabled = True
    results = []
    for i, part in enumerate(re.split(r'mul\(\d+,\d+\)', memory)):
        if i < len(mul_instructions):
            if do_regex.search(part):
                enabled = True
            if dont_regex.search(part):
                enabled = False
                
            if enabled:
                x, y = map(int, re.findall(r'\d+', mul_instructions[i]))
                results.append(x * y)
    
    return sum(results)

with open('input.txt', 'r') as file:
    puzzle_input = file.read()

result = parse_memory(puzzle_input)
print(result)

