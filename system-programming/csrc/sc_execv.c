#include <unistd.h>

// execはexecvの他にもexeclp,execveなどがある

int main(){
    char *argv[3];
    argv[0] = "lsf";
    argv[1] = "-a";
    argv[2] = NULL;
    execv("/bin/ls", argv);
    return 0;
}