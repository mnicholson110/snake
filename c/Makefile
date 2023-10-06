build: ./src/snake.c
	mkdir -p ./bin
	gcc -Wall -std=c99 ./src/*.c -lSDL2 -lSDL2_ttf -o ./bin/snake

debug: ./src/snake.c
	mkdir -p ./debug
	gcc -Wall -g -std=c99 ./src/*.c -lSDL2 -lSDL2_ttf -o ./debug/snake

run:
	./bin/snake

run-debug:
	./debug/snake

clean:
	rm -rf ./debug
	rm -rf ./bin
