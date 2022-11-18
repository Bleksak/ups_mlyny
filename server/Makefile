CXX = clang++
CXXFLAGS = -Wall -Wextra -pedantic -std=c++17

BUILD = bin
SRC = src

BIN = server

HPP = machine/machine.hpp machine/state.hpp server/socket.hpp game/game.hpp game/player.hpp
CPP = $(HPP:.hpp=.cpp) main.cpp

OBJLIST = $(CPP:.cpp=.o)
OBJ = $(addprefix $(BUILD)/, $(OBJLIST))

.PHONY : all

all : $(BIN)

$(BUILD)/main.o : $(SRC)/main.cpp
	@mkdir -p $(dir $@)
	@echo "[CC] $@"
	@$(CXX) -c $(CXXFLAGS) $< -o $@
    
$(BUILD)/%.o: $(SRC)/%.cpp
	@mkdir -p $(dir $@)
	@echo "[CC] $@"
	@$(CXX) -c $(CXXFLAGS) $< -o $@

$(BIN): $(OBJ)
	$(CXX) $^ -o $@
	
run: $(BIN)
	./$(BIN)
