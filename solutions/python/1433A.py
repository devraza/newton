t = int(input())

for _ in range(t):
    resident = input()
    pressed = 0

    pressed += int((int(resident[0])-1)*10 + (len(resident)/2)*(len(resident)+1))
    print(pressed)
