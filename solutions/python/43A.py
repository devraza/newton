n = int(input())

goals = {}

for _ in range(n):
    team = input()
    if team not in goals:
        goals[team] = 0
    goals[team] += 1

print(max(goals, key=goals.get))
