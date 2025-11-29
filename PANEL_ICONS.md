# Panel Tray Icons

This directory contains system tray/panel icons for common applications.

## Added Icons

### Steam
- `steam_tray_mono.svg` - Steam system tray icon

### Spotify
- `spotify-indicator.svg` - Spotify tray indicator
- `spotify-linux-32.svg` - Alternative Spotify icon

### Discord
- `discord-tray.svg` - Default Discord icon
- `discord-tray-connected.svg` - Connected to voice channel
- `discord-tray-speaking.svg` - Speaking in voice
- `discord-tray-muted.svg` - Microphone muted
- `discord-tray-deafened.svg` - Deafened
- `discord-tray-unread.svg` - Unread messages

## Why These Icons?

The official COSMIC icon theme was missing panel/tray icon support, causing applications to fall back to other themes (like Papirus) with inconsistent rendering. These icons:

- Use `fill:currentColor` for automatic light/dark panel theme support
- Are properly sized (16x16, 22x22, 24x24) for sharp rendering
- Follow the same monochrome style as other COSMIC panel elements

## Icon Format

All panel icons follow this structure:

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16">
 <defs>
  <style id="current-color-scheme" type="text/css">
   .ColorScheme-Text { color:#dfdfdf; }
  </style>
 </defs>
 <path style="fill:currentColor" class="ColorScheme-Text" d="..."/>
</svg>
```

The `fill:currentColor` CSS property allows the icon to inherit the panel's text color, ensuring it works with both light and dark themes.

## Source

Icons are sourced from the [Papirus Icon Theme](https://github.com/PapirusDevelopmentTeam/papirus-icon-theme), which is licensed under GPL-3.0.

## Adding More Icons

To add more panel tray icons:

1. Find the icon in Papirus (or create your own following the format above)
2. Add it to `16x16/panel/`, `22x22/panel/`, and `24x24/panel/`
3. Ensure it uses `fill:currentColor` for theme support
4. Update this README with the new icon

## Testing

After adding icons:

```bash
# Update icon cache
gtk-update-icon-cache /path/to/cosmic-icons

# Restart the COSMIC panel
pkill -9 cosmic-panel
```
