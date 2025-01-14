t = int(input())

sizes = []

def square(size):
    size.sort()
    if size[0] == size[1]:
        return (size[0]*2)**2
    if size[0]*2 <= size[1]:
        return max(size)**2
    while size[1] % size[0] != 0:
        size[1] += 1
    return size[1]**2;

for i in range(t):
    size = [int(x) for x in input().split(" ")]
    print(square(size))

"""
def check():
    for i in range(1, 101):
        for j in range(1, 101):
            if square([i,j]) == 36 and i != 6 and j != 6:
                print(i,j)
check()
"""
