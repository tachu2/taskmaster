IMAGE_NAME := supervisor-test
CONTAINER_NAME := supervisor-test-container

.PHONY: all
all: build run

.PHONY: build
build:
	docker build -t $(IMAGE_NAME) .

.PHONY: run
run: build
	docker run -d  --name $(CONTAINER_NAME) --rm $(IMAGE_NAME)

.PHONY: shell
shell: run
	docker exec -it $(CONTAINER_NAME) bash

.PHONY: re
re: down run

.PHONY: down
down:
	docker stop $(CONTAINER_NAME) || true

.PHONY: clean
clean:
	docker rm $(CONTAINER_NAME)
	docker rmi $(IMAGE_NAME)
