# Theme Switcher Implementation

## Overview
Successfully created a new **CLI theme** inspired by Claude Code CLI with a dark terminal aesthetic, and implemented a **theme switcher** in both the CLI and Liquidweb themes.

## New CLI Theme

### Features
- **Dark Terminal Aesthetic**: Based on Claude Code CLI style with a dark color scheme
- **Cyan/Blue Accent Colors**: Primary color is `#00d9ff` (cyan) with dark backgrounds (`#0a0e27`)
- **Monospace Fonts**: Uses `Fira Code`, `JetBrains Mono` for authentic terminal feel
- **Glowing Effects**: Subtle glow/shadow effects for modern depth
- **Glow Borders**: Cyan accent borders on interactive elements
- **Terminal-style Typography**: All-caps labels and letter-spacing for authenticity

### Color Palette
- **Background**: `#0a0e27` (dark navy)
- **Secondary Background**: `#111729`
- **Primary Accent**: `#00d9ff` (cyan)
- **Secondary Accent**: `#0099cc`
- **Text Primary**: `#e1e6f0` (light blue-gray)
- **Text Secondary**: `#8892b0` (muted blue-gray)
- **Borders**: `#1e2749` (dark blue)

## Location
- **CLI Theme**: `/home/mouzi/spn21.github.io/themes/cli/`
- **Liquidweb Theme**: `/home/mouzi/spn21.github.io/themes/liquidweb/` (updated)

## Files Created

### CLI Theme Structure
```
themes/cli/
├── theme.toml
├── templates/
│   ├── base.html
│   ├── index.html
│   ├── blog.html
│   ├── blog-page.html
│   └── about-page.html
└── static/
    └── css/
        └── cli.css
```

## Theme Switcher Implementation

### How It Works
1. **Theme Switcher Button**: Located in the navbar after "关于 (About)" link
   - Shows 🌙 (moon) icon in CLI theme - clicks to switch to Liquidweb
   - Shows 💻 (laptop) icon in Liquidweb theme - clicks to switch to CLI

2. **LocalStorage Persistence**: Theme preference is saved to browser's localStorage as `selectedTheme`
   - CLI theme: `localStorage.setItem('selectedTheme', 'cli')`
   - Liquidweb theme: `localStorage.setItem('selectedTheme', 'liquidweb')`

3. **Page Reload**: Clicking the switcher reloads the page to apply the new theme

### Updated Files
- **Liquidweb base.html**: Added theme-switcher button and functionality
- **Liquidweb CSS**: Added `.theme-switcher` styles
- **CLI base.html**: Added theme-switcher button with same functionality

## Usage

### To Use the New CLI Theme
1. Update `config.toml` in the blog root:
   ```toml
   theme = "cli"
   ```

2. Or keep current theme and users can switch using the navbar button

### Theme Switching
Users can click the theme switcher button (🌙 or 💻) in the navbar to toggle between CLI and Liquidweb themes. Their preference is saved automatically.

## Customization Options

### To Modify CLI Theme Colors
Edit `/home/mouzi/spn21.github.io/themes/cli/static/css/cli.css`:
- Adjust CSS variables at the top of the file under `:root`
- Change glow effects, borders, and accent colors as needed

### To Modify Liquidweb Theme Button Style
Edit `/home/mouzi/spn21.github.io/themes/liquidweb/static/css/liquidweb.css`:
- Find `.theme-switcher` class for button styling

## Browser Compatibility
- Uses CSS Grid, Flexbox (modern browsers)
- LocalStorage API (IE8+)
- Works on desktop and mobile screens (responsive design included)

## Notes
- Both themes share the same content structure and templates
- Only CSS styling differs between themes
- Theme preference persists across browser sessions
- No server-side changes required
