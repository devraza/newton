t = int(input())

for _ in range(t):
    b = list(input())
    original = b[:]
    b.pop(0)
    b.pop(len(b)-1)

    copied = []
    for idx, i in enumerate(b):
        if idx % 2 == 0:
            copied.append(i)

    print(f"{original[0]}{''.join(c for c in copied)}{original[len(original)-1]}")
