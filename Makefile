


.PHONY: build-container
build-container:
	podman build -t dictionary-rs --network=host -f build/container/Containerfile .	
