with open('input.txt') as input_file:
    lines = input_file.readlines()

numbers = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine'
]
plain_numbers = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
numbers_dict = {
    'one' : '1',
    'two': '2',
    'three': '3',
    'four' : '4',
    'five': '5',
    'six': '6',
    'seven': '7',
    'eight': '8',
    'nine': '9'
}

result = 0
for line in lines:
    for nd in numbers_dict:
        line = line.replace(nd, nd+numbers_dict[nd]+nd)
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
