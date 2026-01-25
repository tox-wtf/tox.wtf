-include config.mk

.PHONY: all clean build serve dev local-serve subdomains

all: build subdomains

clean:
	rm -rf target/site target/subdomains target/tmp
	$(MAKE) -C subdomains/lfs clean

purge:
	rm -rf target

target/tmp/pages/index.html: pages
	mkdir -pv target/tmp/pages
	cp -af pages -T target/tmp/pages
	for badge in s/88x31/*; do \
		echo "<img width=88 height=31 src=\"/$$badge\">" >> target/tmp/pages/imgs/88x31.html; \
	done; \
	echo "" >> target/tmp/pages/imgs/88x31.html;
	echo "{% endblock %}" >> target/tmp/pages/imgs/88x31.html;

build: Cargo.toml subdomains target/tmp/pages/index.html
	cargo run
	cp -af s target/site
	cp -f target/tmp/pages/robots.txt target/site/robots.txt

# tidy: build subdomains
# 	find target/site -type f -iname '*.html' -print0 | xargs -0 tidy -m -config tidyconf
# 	find target/subdomains -type f -maxdepth 1 -iname '*.html' -print0 | xargs -0 tidy -m -config tidyconf

subdomains: subdomain-man subdomain-vat subdomain-lfs

subdomain-lfs:
	$(MAKE) -C subdomains/lfs

subdomain-man:
	$(MAKE) -C subdomains/man

subdomain-vat:
	$(MAKE) -C subdomains/vat

serve: build
	caddy run --config Caddyfile

local-serve: build
	caddy run --config Caddyfile.local

dev: clean build local-serve
