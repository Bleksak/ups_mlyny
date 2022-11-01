CC = clang
CFLAGS = -Wall -Wextra -pedantic
BIN = server.exe
OBJ = src/main.o src/board.o src/game.o

%.o: %.cpp
	$(CC) -c $(CFLAGS) $< -o $@

$(BIN): $(OBJ)
	$(CC) $^ -o $@
