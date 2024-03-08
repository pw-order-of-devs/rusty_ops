build server:
	docker build -t rusty-server -f rusty_server/Dockerfile .

build web:
	docker build -t rusty-web -f rusty_web/Dockerfile .
