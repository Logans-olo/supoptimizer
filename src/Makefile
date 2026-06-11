# ---- Configuration ----------------------------------------------------------
CXX      ?= g++
CXXFLAGS ?= -std=c++17 -Wall -Wextra -O2
PYTHON   ?= python3

BIN      := dag_builder      # C++ executable
SOLVER   := solve.py         # Python solver script
DAG_FILE := dag.json         # interchange file shared by both sides

# Auto-discover sources so you don't edit this when adding files.
SRCS     := $(wildcard *.cpp)
OBJS     := $(SRCS:.cpp=.o)
HEADERS  := $(wildcard *.hpp *.h)

# ---- Targets ----------------------------------------------------------------
.PHONY: all run clean

all: $(BIN)

# Link objects into the executable
$(BIN): $(OBJS)
	$(CXX) $(CXXFLAGS) -o $@ $^

# Compile each .cpp; rebuild if any header changes
%.o: %.cpp $(HEADERS)
	$(CXX) $(CXXFLAGS) -c $< -o $@

# End-to-end: C++ emits the DAG, Python solves it
run: $(BIN) $(SOLVER)
	./$(BIN) $(DAG_FILE)
	$(PYTHON) $(SOLVER) $(DAG_FILE)

clean:
	rm -f $(OBJS) $(BIN) $(DAG_FILE)
