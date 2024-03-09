build_server:
	docker build -t rusty-server -f rusty_server/Dockerfile .

build_web:
	docker build -t rusty-web -f rusty_web/Dockerfile .
