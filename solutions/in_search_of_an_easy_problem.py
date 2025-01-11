n = input()
opinions = [int(x) for x in input().split(' ')]

if 1 in opinions:
    print("HARD")
else:
    print("EASY")
