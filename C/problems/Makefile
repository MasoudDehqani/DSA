CC = gcc
CFLAGS = -std=c17 -Wall -Wextra -Wpedantic -Wshadow -Wconversion -Wsign-conversion -Wuninitialized -Werror -fdiagnostics-color=always

SRC = custom_pow.c
OUT = custom_pow

all: $(OUT)

$(OUT): $(SRC)
	$(CC) $(CFLAGS) $(SRC) -o $(OUT)

run: $(OUT)
	./$(OUT)

clean:
	rm -f $(OUT)