#include <stdio.h>
#include <sys/wait.h>
#include <stdlib.h>

int main() {
    int i,status,pid;
    if(0 == (pid=fork())) {
        for(int i=0;i<100;i++){
            printf("%d ...",i);
        }
        exit(0);
    } else {
        int p = wait(&status);
        // exitコールによる終了の場合
        // | exit()の戻り値 |              |
        // |x|x|x|x|x|x|x|x|0|0|0|0|0|0|0|0|
        printf("pid %d p %d retrun value %d\n",pid,p,status);
    }
}