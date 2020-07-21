CC = gcc
CFLAGS = -W -Wall -Werror -Wextra
OBJ = build/main.o build/version.o

build : $(OBJ)
	gcc $(OBJ) -o wmanager

build/main.o : src/main.c
	$(CC) -o build/main.o -c src/main.c $(CFLAGS)

build/version.o : core/version.c
	$(CC) -o build/version.o -c core/version.c $(CFLAGS)