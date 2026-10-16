# color-groups

Computes the colors of prompt segment groups, for the zsh prompt
(`.p10k.zsh` of the `zsh` module) and the Claude Code status line of the
`ai-prompts` module. One spec line describes a group, and `color-groups --help`
lists the fields. `color-groups --check` also prints the colors and checks the
contrast, lightest-first, and neighbor rules below; the code enforces the
lightness bounds.

## Model

Colors are computed in OKLCH, whose lightness approximates perceived lightness
across hues. A group is a matrix with one row per segment and a background and a
text column. The first segment's background is the input, except for `darken`,
which starts from its last, darkest segment. The relation fills the other rows:

- `flip`: later segments are dark and saturated in the group color, each a
  little lighter than the one before.
- `shade`: later segments are a darker shade of the first.
- `darken`: each earlier segment is a fixed step lighter and less saturated than
  the one after it, so that the steps stay visible near the lightness bound.

The text of a segment is solved for an APCA contrast target against its own
background. All later segments share one text color, which meets its target on
the hardest of their backgrounds when the target is reachable; `--check` reports
a target that is not. A `dX` target sets the text lightness to the background
lightness plus X, which can be negative. Use it for contrast too low for APCA to
measure, or for a fixed lightness distance.

## Rules

- Lightness stays between 0.06 and 0.96: no color is pure black or white.
- Each text with an APCA target meets it; a `dX` target is not checked.
- The first segment is the lightest of its group.
- Neighbors do not share a color: the text of a segment differs from the
  backgrounds next to it.

## Sources

- [OKLab](https://bottosson.github.io/posts/oklab/), Bjorn Ottosson: the
  conversion matrices
- [CSS Color 4](https://www.w3.org/TR/css-color-4/): gamut mapping by chroma
  reduction, simplified here, and the deltaEOK color distance
- [APCA-W3](https://github.com/Myndex/apca-w3): contrast constants
- [coloralgorithm](https://github.com/arielsalminen/coloralgorithm): the start,
  end, and easing curve per channel
