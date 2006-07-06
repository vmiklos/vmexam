all: config

config: config.in FORCE
	cat config.in |sed 's/$foo/bar/' >config

FORCE:
