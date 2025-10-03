# osnova-launcher (built‑in GUI module)

Architecture Update (2025-10-03): The Launcher is built into the Osnova shell GUI and is no longer a separately packaged frontend component.

## Layout

App icons are displayed in a grid by retreiving them from the local app cache.
Selecting an app will launch the application and render it into its own tab.

### Desktop Layout
- Continuously scrolling grid of app icons in a window
- Icons arranged in a grid pattern with consistent spacing
- Click-and-drag interaction to reorder icons
- Icons snap to the closest grid cell on drop
- Scrollbar or trackpad/mouse wheel scrolling for navigation

### Mobile Layout (Android/iOS)
- Paginated grid of app icons (one screen per page)
- Swipe left/right to navigate between pages
- Long-press (>= 500ms) to enter reorder mode
- Drag icons to reposition; other icons reshuffle based on target location
- Icons snap to the closest grid cell on drop
- Pages added/removed dynamically as icons are added/removed
- Page indicator dots at the bottom showing current page

## Functionality

### Core Features
1. **Display Apps**: Fetch list of installed apps via `apps.list` OpenRPC method and display icons in grid
2. **Launch Apps**: On icon tap/click, call `apps.launch` with appId, which loads manifest and opens app in new tab/window
3. **Icon Management**: Icons fetched from manifest.iconUri (ant:// Autonomi addresses); fallback to default icon if unavailable
4. **Reordering**: 
   - Desktop: Click-and-drag to reorder; continuous grid with scrolling
   - Mobile: Long-press to enter reorder mode; drag to reposition; swipe pages
5. **Layout Persistence**: 
   - Save icon order via `launcher.setLayout` OpenRPC method
   - Changes debounced and saved within 1s of drop
   - Layout persisted per-identity and restored on relaunch via `launcher.getLayout`
6. **Loading States**: Display loading spinner during app launch; show errors if manifest invalid or components unavailable
7. **Error Handling**: Show clear error messages for missing/invalid components, network issues, or manifest problems

### User Interactions
- **Single tap/click on icon**: Launch the corresponding app
- **Long-press (mobile >= 500ms)**: Enter reorder mode
- **Drag (reorder mode)**: Move icon to new position; other icons reshuffle
- **Swipe left/right (mobile)**: Navigate between pages
- **Drop (reorder mode)**: Snap to grid; persist new order via OpenRPC call

### Default Screen Behavior
The App Launcher is the default screen when launching the Osnova shell. The shell automatically loads the configured launcher manifest (from config) and renders it on startup.

### Visual Design
- Icons: 1024x1024 PNG from manifest.iconUri, dynamically scaled to fix in the grid
- Grid: Responsive sizing based on screen dimensions
- Theme: Follows system theme (light/dark mode from ui.getTheme)
- Animations: Smooth transitions for reordering, page swipes, and icon movements
