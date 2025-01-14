a = input().split(' ')
s = input()

count = 0
for i in s:
    count += int(a[int(i)-1])
print(count)
