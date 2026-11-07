# [shfmt](https://github.com/mvdan/sh)

Shell formatter and parser.

No apt-get script: Debian 12 and Ubuntu 22.04 package shfmt releases before
3.8, which do not know `--apply-ignore` or the `[[shell]]` section of
`.editorconfig`. The format hook of the `ai-prompts` module needs both.
