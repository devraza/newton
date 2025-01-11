t = int(input())

for _ in range(t):
    a, b = [list(x) for x in input().split()]
    
    a[0], b[0] = b[0], a[0]

    print(''.join(c for c in a),''.join(c for c in b))
