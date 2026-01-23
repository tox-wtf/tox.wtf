all: build

../../target/tmp/man:
	find man -mindepth 2 -type f -name '*.[1-8]*' -exec install -vDm644 -t ../../target/tmp/man/ {} +
	./deref.sh

../../target/subdomains/man:
	mkdir -pv ../../target/subdomains/man

build: ../../target/tmp/man ../../target/subdomains/man
	rsync -avu ../../target/tmp/man/ ../../target/subdomains/man/
	cp -af s ../../target/subdomains/man
