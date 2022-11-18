CXX = clang++
CXXFLAGS = -Wall -Wextra -pedantic -std=c++17
BIN = server
OBJ = src/main.o src/board.o src/server.o src/command.o src/state_machine/machine.cpp src/state_machine/state.cpp src/game_destroyer.cpp

run: $(BIN)
	./$(BIN)

%.o: %.cpp
	$(CXX) -c $(CXXFLAGS) $< -o $@

$(BIN): $(OBJ)
	$(CXX) $^ -o $@
