# Compiler and flags
CXX = g++
CXXFLAGS = -Wall -Wextra -std=c++23 -Isrc -Itest

# Targets
APP_TARGET = main
TEST_TARGET = tests

# Source files
APP_SRCS = src/main.cpp src/expression.cpp
TEST_SRCS = test/test_expression.cpp test/test_infra.cpp src/expression.cpp

# Default target
all: $(APP_TARGET)

# Build application
$(APP_TARGET): $(APP_SRCS)
	$(CXX) $(CXXFLAGS) -o $@ $(APP_SRCS)

# Build tests
$(TEST_TARGET): $(TEST_SRCS)
	$(CXX) $(CXXFLAGS) -o $@ $(TEST_SRCS)

# Run tests
test: $(TEST_TARGET)
	./$(TEST_TARGET)

# Clean
clean:
	rm -f $(APP_TARGET) $(TEST_TARGET)