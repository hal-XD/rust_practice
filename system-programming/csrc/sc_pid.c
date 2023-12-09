// UNIXのプロセスはカーネルが管理するために様々なIDをもつ
// プロセスID: psで確認できる。プロセスに割り当てられるID
// プロセスグループID: プロセスIDとプロセスグループIDが等しい場合はプロセスリーダー
// ターミナルグループ:
// リアルユーザーID:
// リアルグループID:
// イフェクティブユーザーID:
// イフェクティブグループ:
// プログラムファイルのユーザーID:
// プログラムファイルのグループID:
//
// uidやeidを変更するシステムコールも存在する
// setuid,setgid,setsid,setpgrp

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/wait.h>

void show_pids(){
    printf("pid               : %d\n", getpid()); // プロセスID
    printf("prosess group id  : %d\n", getpgrp());// プロセスグループID
    printf("parent pid        : %d\n", getppid());// 親プロセスID
    printf("Real User ID      : %d\n", getuid()); // リアルユーザーID
    printf("Effective User Id : %d\n", geteuid());// エフェクティブユーザーID
    printf("Real Group ID     : %d\n", getgid()); // リアルグループID
    printf("Effective Group ID: %d\n", getegid());// エフェクティブグループID
}

int main(){
    int pid,status;
    if ((pid = fork()) == 0) {
        if ((pid = fork()) == 0) {
            printf("grandchild. fork pid=[%d]\n", pid);
            show_pids();
            exit(0);
        }
        wait(&status);
        printf("child. fork pid=[%d]\n", pid);
        show_pids();
        exit(0);
    } else {
        wait(&status);
        printf("parent fork pid=[%d]\n", pid);
        show_pids();
    }
    return 0;
}