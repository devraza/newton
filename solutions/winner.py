n = int(input())

scores = {}
orders = []
for _ in range(n):
    name, score = input().split(' ')

    orders.append([name, int(score)])

    if name not in scores:
        scores[name] = []
    scores[name].append(int(score))

totals = []

for k,v in scores.items():
    totals.append(sum(v))

winval = max(totals)

winners = [k for k,v in scores.items() if sum(v) == winval]

if len(winners) < 1:
    print(winner[0])
else:
    for idx, i in enumerate(orders):
        if i[0] not in winners:
            del orders[idx]
    for idx, i in enumerate(orders):
        if i[0] not in winners:
            del orders[idx]

    print(orders)

    winner = ''
    durations = {}
    for i in orders:
        if i[0] not in durations:
            durations[i[0]] = 0
        durations[i[0]] += i[1]
        if durations[i[0]] == winval:
            winner = i[0]
            break

    print(winner)
