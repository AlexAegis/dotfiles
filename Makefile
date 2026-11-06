# To run a single test: `make test/1.test`
# To run all tests: `make test`

MAKEFLAGS += -k # keep running on failure
MAKEFLAGS += -j1 # explicitly run on a single thread,
# more would cause problems on coverage report merging

SHELL := /bin/sh

.PHONY: lint format format-check clean

# Prints the shell files, one on each line: the tracked files with a shell
# extension, and the ones whose first line runs sh, bash, dash or ksh. Symlinks
# are skipped; their tracked targets are listed. Fails outside a Git work tree.
shell_files = tracked=$$(git -c core.quotePath=false ls-files) && \
	printf '%s\n' "$$tracked" | while IFS= read -r f; do \
		[ -f "$$f" ] && [ ! -L "$$f" ] || continue; \
		case $$f in \
		*.sh | *.bash | *.ksh | *.bashrc | *.bash_profile | *.bash_login | \
		*.bash_logout) ;; \
		*) head -n 1 "$$f" | \
			grep -qE '^\#!.*[/ ](sh|bash|dash|ksh)([[:space:]]|$$)' || continue ;; \
		esac; \
		printf '%s\n' "$$f"; \
	done

# Runs the command that follows it on the shell files. Stops when git fails or
# lists no shell file, so that a check never passes on an empty list.
on_shell_files = files=$$($(shell_files)) && [ -n "$$files" ] || \
		{ echo 'No shell files listed' >&2; exit 1; }; \
	printf '%s\n' "$$files" | tr '\n' '\0' | xargs -0

lint:
	@$(on_shell_files) shellcheck --
	@echo "Success, all lints passed."

# The format targets name the files, so they need --apply-ignore to skip the
# files that an ignore section of .editorconfig names
format:
	@$(on_shell_files) shfmt -w --apply-ignore --

format-check:
	@$(on_shell_files) shfmt -d --apply-ignore --
	@echo "Success, all shell files are formatted."

clean:
	@echo "Clean"
	rm -f .tarhash
