docker-build: ## Builds a docker image container with a Rust environment
	docker build -t anamorph-to-panorama -f Dockerfile .

docker-shell: ## Starts a docker container with a Rust environment
	docker run --rm -it -v `pwd`:/home anamorph-to-panorama /bin/sh
