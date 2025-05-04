t = int(input())

for _ in range(t):
    nums = list(map(int, input().split(' ')))

    if min(nums) == max(nums):
        print("Yes")
    else:
        print("No")
