// プロセスのプライオリティ値
// nice値 : プロセスの優先度

#include <unistd.h>
#include <stdlib.h>

int main(){
    //rtprio(getpid(), 10);
    nice(10);
    return 0;
}