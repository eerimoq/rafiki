.PHONY: all clean

all:
	$(MAKE) -f sphinx.mk html
	echo "Run 'firefox `readlink -f _build/html/index.html`' to view the generated documentation."

clean:
	$(MAKE) -f sphinx.mk clean
