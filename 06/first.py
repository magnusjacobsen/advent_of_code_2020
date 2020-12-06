with open('input', 'r') as file:
    lines = file.readlines()
    current = ''
    count = 0

    for line in lines:
        line = line.strip()
        if line == '':
            count += len(''.join(set(current)))
            current = ''
        else:
            current += line
    count += len(''.join(set(current)))
    print(count)