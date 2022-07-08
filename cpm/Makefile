build: cpm

cpm: Makefile main.go
	go build

check: build check-format check-lint
	@echo "make check: ok"

check-lint:
	golint -set_exit_status

check-format:
	[ -z "$(shell gofmt -l main.go)" ]
