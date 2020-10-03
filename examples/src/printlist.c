#include <stdio.h>
#include "printlist.h"

void printlist(int *list, int length) {
	for (int i = 0; i < length; ++i) {
		printf("[%d] ", list[i]);
	}
	printf("\n");
}
