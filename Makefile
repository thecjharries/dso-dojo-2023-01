FILE=solution-stack.yaml

.PHONY: all
all:

.PHONY: redis
redis:
	docker run --rm --detach --publish 6379:6379 --name redis redis

.PHONY: dependencies
dependencies:
	cd Dockerfiles && $(MAKE) all

.PHONY: stack
stack: dependencies
	if docker-compose ls | grep -q $(FILE); then \
		docker-compose --file $(FILE) down; \
	else \
		docker-compose --file $(FILE) up --detach; \
	fi
