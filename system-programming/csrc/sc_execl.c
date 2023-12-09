#include <unistd.h>

main(){
    execl("/bin/ls","lsf","-a",0);
}