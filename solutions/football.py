positions = input()

followings = []
for idx,i in enumerate(positions):
    following = 0
    for j in range(idx,len(positions)):
        if positions[j] == i:
            following += 1
        else:
            break
    followings.append(following)

if max(followings) >= 7:
    print("YES")
else:
    print("NO")
