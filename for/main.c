#include <stdio.h>

int main(void) {
  int max = 10;
  int count = 0;

  for (int i = 0; i < max; i++) {
    count++;
  }

  printf("%d\n", count);
}
