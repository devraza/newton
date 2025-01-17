#include <stdio.h>
#include <string.h>

int main() {
  int n,i;
  scanf("%d", &n);

  getchar();

  int count = 0;

  for (i = 0; i < n; i++) {
    char operation[4];
    scanf("%s", operation);

    if (strcmp(operation, "X++") == 0 || strcmp(operation, "++X") == 0) {
      count += 1; 
    } else if (strcmp(operation, "X--") == 0 || strcmp(operation, "--X") == 0) {
      count -= 1;
    }
  }
  
  printf("%d\n", count);

  return 0;
}
