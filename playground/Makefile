IMAGE_NAME := supervisor-test
CONTAINER_NAME := supervisor-test-container

.PHONY: all
all: build run

.PHONY: build
build:
	docker build -t $(IMAGE_NAME) .

.PHONY: run
run: build
	docker run -d -p 9001:9001 --name $(CONTAINER_NAME) --rm $(IMAGE_NAME)

.PHONY: shell
shell:
	docker exec -it $(CONTAINER_NAME) bash

.PHONY: re
re: down run

.PHONY: down
down:
	docker stop $(CONTAINER_NAME) || true

.PHONY: clean
clean: down
	docker rm $(CONTAINER_NAME)
	docker rmi $(IMAGE_NAME)
