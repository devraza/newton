t = int(input())

for _ in range(t):
    a,b = [int(x) for x in input().split(" ")]
    minimum = b
    for c in range(a,b+1):
        value = (c-a)+(b-c)
        if value < minimum:
            minimum = value
    print(minimum)
