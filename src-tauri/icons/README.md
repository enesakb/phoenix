# Icons

Placeholder icons for Phoenix. Replace these with branded artwork during Week 8 launch prep.

Required files (per `tauri.conf.json`):
- `32x32.png`
- `128x128.png`
- `icon.ico`

To regenerate from a single source PNG, use Tauri's built-in helper:

```bash
cargo install tauri-cli --version "^2"
cargo tauri icon path/to/source.png
```

Until brand assets exist, generate placeholders with ImageMagick:

```bash
convert -size 32x32 xc:#ff6b35 -draw "text 4,22 'P'" 32x32.png
convert -size 128x128 xc:#ff6b35 -draw "text 35,90 'P'" 128x128.png
convert 128x128.png icon.ico
```
