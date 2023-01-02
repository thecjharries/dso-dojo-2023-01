FILE=solution-stack.yaml

.PHONY: all
all:

.PHONY: redis
redis:
	docker run --rm --detach --publish 6379:6379 --name redis redis

.PHONY: nginx
nginx:
	docker build --tag thecjharries/dso-dojo-2023-01-reverseproxy:latest --file ./Dockerfiles/nginx.Dockerfile .

.PHONY: cache-server
cache-server:
	docker build --tag thecjharries/dso-dojo-2023-01-cacheserver:latest --file ./Dockerfiles/cache-server.Dockerfile .

.PHONY: dependencies
dependencies: nginx cache-server

.PHONY: stack
stack: dependencies
	if docker-compose ls | grep -q $(FILE); then \
		docker-compose --file $(FILE) down; \
	else \
		docker-compose --file $(FILE) up --detach; \
	fi
