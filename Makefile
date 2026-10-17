# To run a single test: `make test/1.test`
# To run all tests: `make test`

MAKEFLAGS += -k # keep running on failure
MAKEFLAGS += -j1 # explicitly run on a single thread,
# more would cause problems on coverage report merging

SHELL := /bin/sh

.PHONY: lint clean

# Shell files are the tracked files with a shell extension, and the ones whose
# first line runs sh, bash, dash, or ksh. Symlinks are skipped; their tracked
# targets are linted.
lint:
	@git -c core.quotePath=false ls-files | while IFS= read -r f; do \
		[ -f "$$f" ] && [ ! -L "$$f" ] || continue; \
		case $$f in \
		*.sh | *.bash | *.ksh | *.bashrc | *.bash_profile | *.bash_login | \
		*.bash_logout) ;; \
		*) head -n 1 "$$f" | \
			grep -qE '^#!.*[/ ](sh|bash|dash|ksh)([[:space:]]|$$)' || continue ;; \
		esac; \
		printf '%s\n' "$$f"; \
	done | tr '\n' '\0' | xargs -0 shellcheck --
	@echo "Success, all lints passed."

clean:
	@echo "Clean"
	rm -f .tarhash
