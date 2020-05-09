# To run a single test: `make test/1.test`
# To run all tests: `make test`

MAKEFLAGS += -k # keep running on failure
MAKEFLAGS += -j1 # explicitly run on a single thread,
# more would cause problems on coverage report merging

SHELL := /bin/sh
COV_COM:='kcov --include-pattern=dot.sh --exclude-pattern=coverage coverage'

all_lint_formats := $(addsuffix .lint, sh dash bash ksh)

.PHONY: lint %.lint clean

list_all_lint_formats:
	@echo $(all_lint_formats)

%.lint:
	@git ls-tree -r master --name-only | \
		grep -e '\.sh$$' -e '\.bash$$' -e '\.ksh$$' -e '\.bashrc$$' -e \
		'\.bash_profile$$' -e '\.bash_login$$' -e '\.bash_logout$$' | \
		xargs shellcheck && \
		echo "Lint $(basename $@) successful" || \
		echo "Lint $@ failed"

lint_all: $(all_lint_formats)

lint: lint_all
	@echo "Success, all lints passed."

clean:
	@echo "Clean"
	rm -f .tarhash
