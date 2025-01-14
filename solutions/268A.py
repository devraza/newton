n = int(input())

uniforms = []

for _ in range(n):
    h, a = input().split(' ')
    uniforms.append([h,a])

count = 0
for idx_i, i in enumerate(uniforms):
    for idx_j, j in enumerate(uniforms):
        if idx_i != idx_j and i[0] == j[1]:
            count += 1
print(count)
