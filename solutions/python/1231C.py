import sys 

n, m = map(int, input().split(' '))

combined = []

for _ in range(n):
    cols = map(int, input().split(' '))

    combined += cols

combined = combined[::-1]

def fail():
    print("-1")
    sys.exit()

check = False
for idx, i in enumerate(combined):
    if i == 0:
        placements = [combined[idx-m], combined[idx+m]]

        if abs(placements[1]-placements[0]) == 1:
            fail()

        if abs(combined[idx+1]-combined[idx-1]) != 1:
            if placements[0] != 0:
                options = [
                    combined[idx-m]-1,
                    combined[idx-1]-1
                ]

                minimum = min(range(len(options)), key=lambda x: options[x])

                if (abs(combined[idx-1]-combined[idx+m]) == 1) or (abs(combined[idx+1]-combined[idx-m]) == 1):
                    print("-1")
                    sys.exit()

                if minimum == 1:
                    combined[idx] = options[minimum]
                elif minimum == 0:
                    combined[idx] = placements[0]-1
            else:
                combined[idx] = combined[idx-1]-1
        else:
            fail()

for i in range(0,len(combined)):
    if (i+1) % m != 0 and (combined[i] <= combined[i+1]):
        fail()
    if i+m < len(combined):
        if combined[i+m] >= combined[i]:
            fail()

print(sum(combined))
