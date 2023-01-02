FILE=solution-stack.yaml

.PHONY: all
all:

.PHONY: dev
dev:
	$(MAKE) docker-compose FILE=./dev-stack.yaml

.PHONY: nginx
nginx:
	docker build --tag thecjharries/dso-dojo-2023-01-reverseproxy:latest --file ./Dockerfiles/nginx.Dockerfile .

.PHONY: cache-server
cache-server:
	docker build --tag thecjharries/dso-dojo-2023-01-cacheserver:latest --file ./Dockerfiles/cache-server.Dockerfile .

.PHONY: dependencies
dependencies: nginx cache-server

.PHONY: solution
solution: dependencies
	$(MAKE) docker-compose FILE=./solution-stack.yaml

.PHONY: docker-compose
docker-compose:
	if docker-compose ls | grep -q $(FILE); then \
		docker-compose --file $(FILE) down; \
	else \
		docker-compose --file $(FILE) up --detach; \
	fi
