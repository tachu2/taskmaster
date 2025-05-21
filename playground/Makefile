IMAGE_NAME := supervisor-test
TAG := latest
CONTAINER_NAME := supervisor-test-container

.PHONY: all
all: build run

.PHONY: build
build:
	docker build -t $(IMAGE_NAME):$(TAG) .

.PHONY: run
run: build
	docker run -d --name $(CONTAINER_NAME) $(IMAGE_NAME):$(TAG)

.PHONY: shell
shell: run
	docker exec -it $(CONTAINER_NAME) bash

.PHONY: re
re: clean run

.PHONY: clean
clean:
	docker stop $(CONTAINER_NAME)

.PHONY: fclean
fclean: clean
	docker rm $(CONTAINER_NAME)
	docker rmi $(IMAGE_NAME):$(TAG)
