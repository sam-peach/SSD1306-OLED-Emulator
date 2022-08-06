#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <windows.h>

int main()
{
  // setvbuf(stdout, NULL, _IONBF, 0);
  int min_number = 1;
  int max_number = 10;
  while (1)
  {
    int num = rand() % (max_number + 1 - min_number) + min_number;
    printf("%i\n", num);
    fflush(stdout);

    Sleep(1000);
  }

  return 0;
}