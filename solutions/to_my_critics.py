from itertools import permutations

t = int(input())

for _ in range(t):
    nums = [int(c) for c in input().split(' ')]
    
    greater = False
    for i in permutations(nums, 2):
        if sum(i) >= 10:
            print("YES")
            greater = True
            break

    if not greater:
        print("NO")
