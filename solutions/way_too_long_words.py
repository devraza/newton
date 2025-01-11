n = int(input())

for _ in range(n):
    word = input()

    if len(word) > 10:
        output = word[0] + str(len(word)-2) + word[len(word)-1]
        print(output)
    else:
        print(word)
