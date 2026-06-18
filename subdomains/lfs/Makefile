all:
	./pull.sh
	./build.sh
	install -vDm644 *.html -t ../../target/subdomains/lfs/
	cp -af s -T ../../target/subdomains/lfs/s

clean:
	rm -rf *lfs/sha *lfs/target
