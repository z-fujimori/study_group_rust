#include <stdio.h>
#include <stdlib.h>

int main(void) {
    int n, l;
    if (scanf("%d %d", &n, &l) != 2) {
        return 1;
    }

    int count = 0;
    for (int i = 0; i < n; i++) {
        int a;
        if (scanf("%d", &a) != 1) {
            return 1;
        }
        if (a >= l) {
            count++;
        }
    }

    printf("%d\n", count);
    return 0;
}
