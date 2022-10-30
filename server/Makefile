CC = gcc
CFLAGS = -Wall -Wextra -pedantic
BIN = server
OBJ = src/main.o src/board.o src/game.o src/error.o src/app.o src/server_messages.o

run: $(BIN)
	./server

%.o: %.c
	$(CC) -c $(CFLAGS) $< -o $@

$(BIN): $(OBJ)
	$(CC) $^ -o $@
