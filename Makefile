build-push:
	docker build . --tag zireael13/aucares:latest
	docker push zireael13/aucares:latest