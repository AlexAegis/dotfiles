# Presets

Presets define a set of modules to be installed using `pont`

The presets folder will be recursively searched through for `*.preset` files

## Usage

Presets can be used with the `+` prefix

### Example

```sh
+dev
```

will use everything in the `dev.preset` file

## Preset file format

Preset files shares the same file format as dependencies. They can even use
other presets.
