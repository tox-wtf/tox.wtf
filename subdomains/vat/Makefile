all: build

build:
	install -vDm644 *.html -t ../../target/subdomains/vat/
	rsync -avu --delete /home/tox/src/vat/p/ ../../target/subdomains/vat/
	cp -af s ../../target/subdomains/vat
