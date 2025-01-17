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

def zero_replace(combined, placements):
    if placements[0] != 0:
        options = [
            combined[idx-m]-1,
            combined[idx-1]-1
        ]

        minimum = min(range(len(options)), key=lambda x: options[x])
    
        diff_forward = abs(combined[idx-1]-combined[idx+m]) == 1
        diff_backward = abs(combined[idx+1]-combined[idx-m]) == 1

        if diff_forward or diff_backward:
            fail()

        if minimum == 1:
            combined[idx] = options[minimum]
        elif minimum == 0:
            combined[idx] = placements[0]-1
    else:
        combined[idx] = combined[idx-1]-1

for idx, i in enumerate(combined):
    if i == 0:
        placements = [combined[idx-m], combined[idx+m]]

        if abs(placements[1]-placements[0]) == 1:
            fail()

        if abs(combined[idx+1]-combined[idx-1]) != 1:
            zero_replace(combined, placements)
        else:
            fail()

for i in range(0,len(combined)):
    if (i+1) % m != 0 and (combined[i] <= combined[i+1]):
        fail()
    if i+m < len(combined):
        if combined[i+m] >= combined[i]:
            fail()

print(sum(combined))
