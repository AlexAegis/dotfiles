#!/bin/sh
# Computes the colors of prompt segment groups in OKLCH from one spec per group.

usage() {
	cat <<'EOF'
Usage: color-groups [--check] < specs

Reads one group per line and prints one "background text" pair of hex colors
per segment. Blank lines and lines starting with # are skipped. Spec fields:

  n relation L C h r Lc0 Lc1 ts0 ts1 th [lift]

- n: segment count
- relation: flip, shade, or darken
- L C h: OKLCH background of the first segment (of the last for darken):
  lightness 0-1, chroma, and hue in degrees. Chroma above the sRGB gamut is
  reduced to fit.
- r: hue rotation of the later segments in degrees; no effect when n is 1
- Lc0, Lc1: APCA contrast of the text on the first and on the later segments.
  Positive means dark text, negative light text. dX instead sets the text
  lightness to the background lightness plus X, which can be negative, for
  contrast too low for APCA to measure or for a fixed lightness distance.
- ts0, ts1: text saturation (chroma / lightness) of the first and later text
- th: text hue, +d or -d for the first background hue + d, or an absolute hue
- lift: optional, flip and shade only: raises the later segments by this
  fraction of the range between lightness 0.15 and the first lightness

--check also prints each color to stderr and checks the rules: each APCA target
is met within 1, the first segment is the lightest, and neighbors do not share
a color.

Exit status: 0, or 1 when --check finds a rule failure, or 2 for an invalid
spec or argument. An invalid spec stops the output after the groups before it.
EOF
}

check=0
case $# in
0) ;;
1)
	case $1 in
	--check) check=1 ;;
	-h | --help)
		usage
		exit 0
		;;
	*)
		usage >&2
		exit 2
		;;
	esac
	;;
*)
	usage >&2
	exit 2
	;;
esac

# The C locale keeps the decimal point a period.
LC_ALL=C exec awk -v check="$check" '
BEGIN {
	# Lightness bounds: no color is pure black or pure white.
	Lmin = 0.06
	Lmax = 0.96
	pi = 3.141592653589793
	# Presets: L position (from Lbase), saturation ratio, and hue steps of r,
	# each as start, end, and easing over the later segments.
	Lbase = 0.15
	preset("flip", "0.08 0.22 linear 1.10 0.95 linear 1 1")
	preset("shade", "0.81 0.65 outQuad 1.15 1.10 linear 1 2")
	# darken counts back from its darkest, last segment.
	P["darken", "step"] = 0.03
	P["darken", "sat"] = 0.6
	num = "^[+-]?([0-9]+[.]?[0-9]*|[.][0-9]+)$"
	failed = 0
	invalid = 0
}

function preset(name, spec,   x, i) {
	split(spec, x, " ")
	for (i = 1; i <= 8; i++) P[name, i] = x[i]
}
function lerp(a, b, t) { return a + (b - a) * t }
function bound(L) { return L < Lmin ? Lmin : L > Lmax ? Lmax : L }
function ease(name, t) { return name == "outQuad" ? 1 - (1 - t) ^ 2 : t }

# sRGB transfer functions, for channels from 0 to 1.
function lin(c) {
	return c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ^ 2.4
}
function enc(c) {
	return c <= 0.0031308 ? 12.92 * c : 1.055 * c ^ (1 / 2.4) - 0.055
}
function cbrt(x) { return x < 0 ? -((-x) ^ (1 / 3)) : x ^ (1 / 3) }

# OKLab matrices by Bjorn Ottosson, https://bottosson.github.io/posts/oklab/
# Sets o[1..3] to the linear sRGB of an OKLCH color.
function oklch2lin(L, C, h, o,   a, b, l, m, s) {
	a = C * cos(h * pi / 180)
	b = C * sin(h * pi / 180)
	l = L + 0.3963377774 * a + 0.2158037573 * b
	m = L - 0.1055613458 * a - 0.0638541728 * b
	s = L - 0.0894841775 * a - 1.2914855480 * b
	l = l * l * l
	m = m * m * m
	s = s * s * s
	o[1] = 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s
	o[2] = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s
	o[3] = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s
}
function ingamut(o,   i) {
	for (i = 1; i <= 3; i++)
		if (o[i] < -1e-6 || o[i] > 1.000001) return 0
	return 1
}

# Sets o[1..3] to the sRGB channels, 0-255, of an OKLCH color, reducing chroma
# to fit the gamut. Returns the chroma used.
function oklch(L, C, h, o,   lo, hi, mid, t, i) {
	oklch2lin(L, C, h, t)
	if (!ingamut(t)) {
		lo = 0
		hi = C
		while (hi - lo > 0.0001) {
			mid = (lo + hi) / 2
			oklch2lin(L, mid, h, t)
			if (ingamut(t)) lo = mid
			else hi = mid
		}
		oklch2lin(L, lo, h, t)
		C = lo
	}
	for (i = 1; i <= 3; i++) {
		t[i] = t[i] < 0 ? 0 : t[i] > 1 ? 1 : t[i]
		o[i] = int(enc(t[i]) * 255 + 0.5)
	}
	return C
}

# Sets o[1..3] to the OKLab coordinates of an sRGB color.
function oklab(c, o,   r, g, b, l, m, s) {
	r = lin(c[1] / 255)
	g = lin(c[2] / 255)
	b = lin(c[3] / 255)
	l = cbrt(0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b)
	m = cbrt(0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b)
	s = cbrt(0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b)
	o[1] = 0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s
	o[2] = 1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s
	o[3] = 0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s
}

# Distance in OKLab, the deltaEOK of CSS Color 4.
function dE(x, y,   a, b) {
	oklab(x, a)
	oklab(y, b)
	return sqrt((a[1] - b[1]) ^ 2 + (a[2] - b[2]) ^ 2 + (a[3] - b[3]) ^ 2)
}

# APCA Lc of fg on bg, https://github.com/Myndex/apca-w3
function apcaY(c,   y) {
	y = 0.2126729 * (c[1] / 255) ^ 2.4 + 0.7151522 * (c[2] / 255) ^ 2.4 \
		+ 0.0721750 * (c[3] / 255) ^ 2.4
	return y < 0.022 ? y + (0.022 - y) ^ 1.414 : y
}
function apca(fg, bg,   yt, yb, s) {
	yt = apcaY(fg)
	yb = apcaY(bg)
	if ((yb > yt ? yb - yt : yt - yb) < 0.0005) return 0
	if (yb > yt) s = 1.14 * (yb ^ 0.56 - yt ^ 0.57)
	else s = 1.14 * (yb ^ 0.65 - yt ^ 0.62)
	if (s < 0.1 && s > -0.1) return 0
	return 100 * (s > 0 ? s - 0.027 : s + 0.027)
}

# Sets T to the text color on B for the target. Returns the text lightness.
function text(target, L, ts, h, B, T,   lo, hi, mid, i, light) {
	if (target ~ /^d/) {
		mid = bound(L + substr(target, 2))
		oklch(mid, ts * mid, h, T)
		return mid
	}
	light = target < 0
	lo = light ? L : Lmin
	hi = light ? Lmax : L
	for (i = 0; i < 30; i++) {
		mid = (lo + hi) / 2
		oklch(mid, ts * mid, h, T)
		if (light ? apca(T, B) <= target : apca(T, B) >= target) {
			if (light) hi = mid
			else lo = mid
		} else if (light) lo = mid
		else hi = mid
	}
	mid = light ? hi : lo
	oklch(mid, ts * mid, h, T)
	return mid
}

function hex(c) { return sprintf("%02x%02x%02x", c[1], c[2], c[3]) }
function copy(from, to) { to[1] = from[1]; to[2] = from[2]; to[3] = from[3] }

function fail(message) {
	printf "  FAIL: %s\n", message > "/dev/stderr"
	failed = 1
}

# Checks that the text on background b meets its APCA target, within 1.
function reached(target, k, f, b,   lc) {
	if (target ~ /^d/) return
	lc = apca(f, b)
	if (target < 0 ? lc > target + 1 : lc < target - 1)
		fail("text of k" k " reaches Lc " sprintf("%.1f", lc) \
			", not " target)
}

function valid(   i) {
	if ($1 !~ /^[1-9][0-9]*$/ || !(($2, 1) in P || ($2, "step") in P))
		return 0
	if (NF != 11 && NF != 12) return 0
	for (i = 3; i <= NF; i++) {
		if (i == 7 || i == 8) {
			if ($i !~ num && !($i ~ /^d/ && substr($i, 2) ~ num)) return 0
		} else if ($i !~ num) return 0
	}
	# Chroma, text saturation, and lift cannot be negative.
	return $4 >= 0 && $9 >= 0 && $10 >= 0 && (NF == 11 || $12 >= 0)
}

/^[ \t]*(#|$)/ { next }

{
	if (!valid()) {
		printf "color-groups: line %d: invalid spec: %s\n", NR, $0 \
			> "/dev/stderr"
		invalid = 1
		exit 2
	}
	n = $1
	rel = $2
	r = $6
	# Derived chroma scales from the input chroma after gamut reduction.
	C0 = oklch(bound($3), $4, $5, b)
	if ((rel, "step") in P) {
		for (k = n - 1; k >= 0; k--) {
			j = n - 1 - k
			L[k] = bound($3 + j * P[rel, "step"])
			C[k] = L[k] * C0 / bound($3) * P[rel, "sat"] ^ j
			H[k] = $5 - r * j
		}
	} else {
		L[0] = bound($3)
		C[0] = C0
		H[0] = $5
		for (k = 1; k < n; k++) {
			t = n > 2 ? (k - 1) / (n - 2) : 0
			pos = lerp(P[rel, 1], P[rel, 2], ease(P[rel, 3], t)) + $12
			L[k] = bound(Lbase + (L[0] - Lbase) * pos)
			C[k] = L[k] * C[0] / L[0] \
				* lerp(P[rel, 4], P[rel, 5], ease(P[rel, 6], t))
			H[k] = H[0] + r * lerp(P[rel, 7], P[rel, 8], t)
		}
	}
	th = $11 ~ /^[+-]/ ? H[0] + $11 : $11
	# Later segments share one text color, solved on the hardest background.
	worst = 0
	light = $8 ~ /^d/ ? substr($8, 2) + 0 > 0 : $8 + 0 < 0
	for (k = 1; k < n; k++)
		if (!worst || (light ? L[k] > L[worst] : L[k] < L[worst])) worst = k
	for (k = 0; k < n; k++) {
		C[k] = oklch(L[k], C[k], H[k], b)
		BG[k, 1] = b[1]
		BG[k, 2] = b[2]
		BG[k, 3] = b[3]
	}
	b[1] = BG[0, 1]; b[2] = BG[0, 2]; b[3] = BG[0, 3]
	TL[0] = text($7, L[0], $9, th, b, T0)
	if (worst) {
		b[1] = BG[worst, 1]; b[2] = BG[worst, 2]; b[3] = BG[worst, 3]
		TL[1] = text($8, L[worst], $10, th, b, T1)
	}
	if (check) printf "%s\n", $0 > "/dev/stderr"
	for (k = 0; k < n; k++) {
		b[1] = BG[k, 1]; b[2] = BG[k, 2]; b[3] = BG[k, 3]
		if (k) copy(T1, f)
		else copy(T0, f)
		print hex(b), hex(f)
		if (check) {
			printf "  k%d bg %s L %.3f C %.3f h %.1f | text %s L %.3f" \
				" | Lc %+.1f\n", k, hex(b), L[k], C[k], H[k], hex(f), \
				TL[k ? 1 : 0], apca(f, b) > "/dev/stderr"
			reached(k ? $8 : $7, k, f, b)
			# Neighbors do not share a color; the first segment is lightest.
			if (k && dE(pf, b) < 0.05)
				fail("text of k" k - 1 " is close to the background of k" k)
			if (k && dE(f, pb) < 0.05)
				fail("text of k" k " is close to the background of k" k - 1)
			if (k && L[k] >= L[0])
				fail("k" k " is not darker than the first segment")
		}
		copy(b, pb)
		copy(f, pf)
	}
}

END {
	if (invalid) exit 2
	if (check && failed) exit 1
}
'
