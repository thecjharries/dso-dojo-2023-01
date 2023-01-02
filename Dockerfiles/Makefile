.PHONY: all
all: nginx

.PHONY: nginx
nginx:
	docker build --tag thecjharries/dso-dojo-2023-01-reverseproxy:latest --file ./nginx.Dockerfile .
