# Compiler and flags
CXX = g++
CXXFLAGS = -Wall -Wextra -std=c++23

# Target and source files
TARGET = main
SRCS = src/main.cpp

# Default target
all: $(TARGET)

# Build target
$(TARGET): $(SRCS)
	$(CXX) $(CXXFLAGS) -o $(TARGET) $(SRCS)

# Clean up build files
clean:
	rm -f $(TARGET)