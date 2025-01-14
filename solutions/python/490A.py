import sys

n = int(input())
skills = [int(x) for x in input().split(' ')]

counts = [0, 0, 0]

for i in range(3):
    counts[i] = skills.count(i+1)

if counts.count(0) == 0:
    print(min(counts))
else:
    print("0")
    sys.exit()

def remove_extras(inp, bound):
    counts = {}
    return [ v if (counts.setdefault(v, 0) < bound and not counts.__setitem__(v, counts[v] + 1)) else 0 for v in inp ]
skills = remove_extras(skills, min(counts))

teams = []
for _ in range(min(counts)):
    team = []
    vals = []
    for idx,i in enumerate(skills):
        if len(team) >= 3:
            break
        if i != 0 and i not in vals:
            team.append(idx+1)
            vals.append(i)
            skills[idx] = 0
    teams.append(team)

"""
teams = []
for _ in range(min(counts)):
    team = []
    for idx,i in enumerate(skills):
        add = true
        for j in teams:
            if idx in j:
                add = false
        for j in team:
            if j[0] == i:
                add = false
        if add:
            team.append((i,idx))
    new_team = []
    for i in team:
        new_team.append(i[1])
    teams.append(new_team)
"""

for i in teams:
    print(' '.join(str(d) for d in i))
