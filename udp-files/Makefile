CC ?= gcc

all: serve client

client:
	$(CC) -o bin/client -Wall src/client.c -O2 -g

serve:
	$(CC) -o bin/serve -Wall src/serve.c -O2 -g

clean:
	rm -rf bin/*
