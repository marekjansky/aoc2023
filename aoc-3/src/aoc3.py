import numpy as np

def gen_surr(x, y, maxx, maxy):
    out = []
    for dy in range(y-1, y+2):
        for dx in range(x-1, x+2):
            if dx >= 0 and dy >= 0 and dx < maxx and dy < maxy:
                out.append((dy, dx))

    return out
        

def main():
    
    data : list[list[str]]= []

    with open("./input.txt") as f:
        for line in f.readlines():
            data.append(list(line.strip()))

    size_x = len(data[0])
    size_y = len(data)

    result_sum = 0

    for idy in range(size_y):
        for idx in range(size_x):
            elmnt = data[idy][idx]
            if elmnt == "*":
                sym_numbers : list[str]= []
                for (y, x) in gen_surr(idx, idy, size_x, size_y):
                    # check surroundings
                    char = data[y][x]
                    if char.isnumeric():
                        # walk the number in both directions
                        number = f"{char}"
                        for dx in range(x+1, size_x):
                            if data[y][dx].isnumeric():
                                number += str(data[y][dx])
                            else:
                                break
                        
                        for dx in range(x-1, -1, -1):
                            if data[y][dx].isnumeric():
                                number = str(data[y][dx]) + number
                            else:
                                break
                        sym_numbers.append(number)
                gear_numbers = list(map(int, set(sym_numbers)))
                if len(gear_numbers) == 2:
                    result_sum += gear_numbers[0] * gear_numbers[1]

    print(result_sum)
                        


if __name__ == "__main__":
    main()