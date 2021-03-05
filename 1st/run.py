
def increment(data):
    for x in range(len(data)):
        if(data[x] < 200):
            continue
        data[x] = 0
        if x >= len(data) - 1:
            data[0] += 1
        else:    
            data[x+1] +=1

def does_match(numbers, data, sum):
    value = 0
    for x in range(len(data)):
        value += numbers[data[x]]
    return value == sum

def calculate(numbers, amount, sum):
    data = {} 
    for i in range(amount):
        data[i] = 0
    while(True):
        increment(data)
        if does_match(numbers, data, sum):
            break
        data[0] +=1
        
    result = 1
    for x in range(len(data)):
        result *= numbers[data[x]]
    return result

if __name__ == "__main__":
    with open("input.txt", "r") as input:
        numbers = [int(i) for i in input.read().splitlines()]
    numbers.sort()
    print("One: ", calculate(numbers, 2, 2020))
    print("two: ", calculate(numbers, 3, 2020))