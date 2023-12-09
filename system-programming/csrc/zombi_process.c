#include <stdio.h>
#include <stdlib.h>

int main()
{
    if(fork() == NULL)
        exit(10);
    else
        for(;;);
}