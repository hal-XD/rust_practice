
// SIGHUPを無視するようにすればシェルが終了しても動き続けるプログラムをつくれる
// abort()は自プロセスにSIGIOT(6)を送信する
// alarm(unsigned int sec) はsecで指定された秒数後に自プロセスにSIGALRMを送る
// pause() シグナルを受けるまでサスペンドする
// kill プロセスにシグナルを送る

#include <stdio.h>
#include <signal.h>
void sigfunc(){
    printf("singnal SIGINT occurerd\n");
}
int main() {
    signal(SIGINT, sigfunc);
    return 0;
}