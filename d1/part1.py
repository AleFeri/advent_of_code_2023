with open('input.txt') as input_file:
    lines = input_file.readlines()

result = 0
for line in lines:
    first_num = ''
    for char in line:
        if char in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']:
            first_num = char
    second_num = ''
    for char in line[::-1]:
        if char in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']:
            second_num = char
    result += int(second_num+first_num)
print(result)
