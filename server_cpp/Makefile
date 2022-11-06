CXX = clang++
CXXFLAGS = -Wall -Wextra -pedantic -std=c++17
BIN = server
OBJ = src/main.o src/board.o src/game.o src/server.o

%.o: %.cpp
	$(CXX) -c $(CXXFLAGS) $< -o $@

$(BIN): $(OBJ)
	$(CXX) $^ -o $@
