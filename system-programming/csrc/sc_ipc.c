
// IPC(Interprocess Communication)
// プロセス間通信の機能
//  - System V IPC(メッセージ機能、共有メモリ、セマフォ機能)
//  - ファイルシステム
//  - シグナル
//  - バークレーソケット

// 共有メモリの操作
//  - shmget()
//  - shmat()
//  - shmdt()
//  - shmctl()

// メッセージの操作
// - msgget()
// - msgsnd()
// - msgrcv()
// - msgctl()

#include <stdio.h>
#include <stdlib.h>

#include <sys/types.h>
#include <sys/ipc.h>
#include <sys/shm.h>

#define SIZE 10

int main(){
    int i,shmid;
    int *p,*head;

    // int10個分の共有メモリを確保
    if ((shmid = shmget(IPC_PRIVATE, SIZE*sizeof(int), 0666)) == EOF) {
        perror("Shmget エラー");
        exit(1);
    }
    if ((head = p = (int *)shmat(shmid,0,0)) == (int *) EOF) {
        perror("shmat error");
        exit(1);
    }
    for (i=0;i!=SIZE;i++) {
        printf("%7d", *p++ = i);
    }
    putchar('\n');
    p = head;
    for(i=0;i!=SIZE;i++) {
        printf("%7d", *p++);
    }
    if (shmdt(head) == EOF) {
        perror("shmdt error");
        exit(1);
    }
    if (shmctl(shmid, IPC_RMID, 0) == EOF) {
        perror("shmctl error");
        exit(1);
    }
    printf("finish\n");
    return 0;
}