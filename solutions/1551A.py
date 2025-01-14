t = int(input())

def check(c1,c2):
    if c1+(2*c2) != n:
        if (c1+1)+(2*c2) == n:
            return c1+1,c2
        elif c1+(2*(c2+1)) == n:
            return c1,c2+1
    else:
        return c1,c2

for _ in range(t):
    n = int(input())
    c1,c2 = [n//3, n//3]
  
    print(' '.join(str(x) for x in check(c1,c2)))
