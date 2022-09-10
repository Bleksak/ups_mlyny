CC = clang
CFLAGS = -Wall -Wextra -pedantic
BIN = server.exe
OBJ = src/main.o src/board.o src/game.o

%.o: %.c
	$(CC) -c $(CFLAGS) $< -o $@

$(BIN): $(OBJ)
	$(CC) $^ -o $@
