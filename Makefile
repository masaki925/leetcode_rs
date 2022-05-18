
N=test

help:
	@echo "make new N=<package name>"

new:
	cargo new $(N)
	cp ./template.rs $(N)/src/main.rs

