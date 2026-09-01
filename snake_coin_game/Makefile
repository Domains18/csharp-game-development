# Variables
NET_VERSION = net8.0
PROJECT_NAME = csharp-game
CONFIG = Debug

# Default target
.PHONY: all
all: build

# Restore dependencies
.PHONY: restore
restore:
	dotnet restore

# Build the project
.PHONY: build
build: restore
	dotnet build --configuration $(CONFIG)

# Run the project
.PHONY: run
run:
	dotnet run --configuration $(CONFIG)

# Run unit tests
.PHONY: test
test:
	dotnet test

# Clean build artifacts
.PHONY: clean
clean:
	dotnet clean
	rm -rf bin/ obj/
