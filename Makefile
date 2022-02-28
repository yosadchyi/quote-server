.PHONY: serve build

run: build
	docker run --name quote-server -p 8080:8080 quote-server:latest

build:
	docker build -f Dockerfile -t quote-server:latest .
