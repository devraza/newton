t = int(input())

for _ in range(t):
    count = int(input())
    weights = [int(x) for x in input().split(" ")]

    if count % 2 != 0 and min(weights) == max(weights):
        print("NO")
        continue
    elif sum(weights) % 2 != 0:
        print("NO")
        continue
    else:
        print("YES")
