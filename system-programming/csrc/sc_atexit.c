#include <stdlib.h>
#include <stdio.h>

void message()
{
    printf("message\n");
}

int main()
{
    atexit(message);
}