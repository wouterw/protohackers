.PHONY: build run

build:
	docker build -t protohackers .

run:
	docker run -t -i --rm protohackers sh
