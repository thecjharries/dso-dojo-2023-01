.PHONY: all
all:

.PHONY: redis
redis:
	docker run --rm --detach --publish 6379:6379 --name redis redis
