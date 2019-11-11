#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *hello(char *name) {
    char c = '!';
    size_t len = strlen(name);
    char *str2 = malloc(len + 1 + 1);
    strcpy(str2, name);
    str2[len] = c;
    str2[len + 1] = '\0';
    return str2;
}

// int main() {
//   char *name = "hello";
//   printf("%s\n", hello(name));
//   return 0;
// }
