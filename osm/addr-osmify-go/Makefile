GO_OBJECTS = \
	addr_osmify.go \
	addr_osmify_test.go \
	main.go \
	urllib.go \


addr-osmify-go: Makefile addr_osmify.go
	go build

check: check-format check-lint check-unit check-headers check-static

check-lint:
	golint -set_exit_status

# Without coverage: 'go test ./...'.
check-unit:
	go build ./...
	courtney -e ./...

check-format:
	[ -z "$(shell gofmt -l $(GO_OBJECTS))" ]

check-headers:
	addlicense -c '$(shell git config user.name)' -ignore '.github/**' -ignore 'guide/book/**' -l mit -s=only -check .

check-static:
	staticcheck ./...
