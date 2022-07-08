GO_OBJECTS = \
	addr_osmify.go \
	addr_osmify_test.go \
	main.go \
	urllib.go \


addr-osmify-go: Makefile addr_osmify.go
	go build

check: check-format check-lint check-race check-unit

check-lint:
	golint -set_exit_status

check-unit:
	./coverage.sh

check-format:
	[ -z "$(shell gofmt -l $(GO_OBJECTS))" ]

check-race:
	go test -race
