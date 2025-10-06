# Mobile UI Design

For mobile OSes, the application should have a clean interface optimized for touch interaction that works for both iOS and Android.

## Layout

### Main Screen Structure
- **Status Bar**: System status bar (iOS/Android)
- **Content Area**: Full-screen app content or launcher grid
- **Bottom Navigation**: 5-icon menu (user configurable)

## Bottom Navigation Menu

### Configuration
- **Icon Count**: Exactly 5 icons
- **User Configurable**: Select which Osnova apps appear
- **Purpose**: Quick access to frequently used apps
- **Persistence**: Saved per user via `nav.setBottomMenu`

### Layout
- Fixed at bottom of screen
- Always visible (except in full-screen mode)
- Icons evenly spaced
- Active tab highlighted

### Icons
- App icons or system icons (home, search, etc.)
- Label below icon (optional, configurable)
- Badge support for notifications (post-MVP)

### Interaction
- Tap icon to switch to that app tab
- Long-press to configure (reorder, replace)
- Visual feedback on tap

## Application Launcher (Mobile)

### Grid Layout
- Paginated grid (one screen per page)
- Typically 4x5 grid (20 icons per page)
- Page indicator dots at bottom
- Swipe gestures to navigate pages

### Interaction
- **Single tap**: Launch application
- **Long-press (>= 500ms)**: Enter reorder mode
- **Drag (reorder mode)**: Move icon to new position
- **Swipe left/right**: Change pages
- **Drop**: Snap to grid cell, save layout

### Reorder Mode
- Icons jiggle slightly (iOS style) or have edit indicator
- Drag icon to reposition
- Other icons reshuffle automatically
- Drag to edge of screen to move between pages
- Tap outside or "Done" button to exit

### Visual Design
- Icons: 1024x1024 PNG, scaled to fit grid
- Touch-friendly spacing (48dp minimum)
- Clear page indicators
- Smooth animations

## Theme Management

### Theme Toggle
- **Location**: Configuration screen
- **Modes**: Light, Dark, System
- **Automatic Switching**: Follows OS theme in System mode
- **Persistence**: Saved via `ui.setTheme`

### Implementation
- Respects iOS/Android system theme APIs
- Real-time theme switching
- Smooth transitions
- All screens update consistently

## Navigation Patterns

### Tab Navigation
- Bottom 5-icon menu for primary navigation
- In-app navigation via app-specific UI
- Back button (Android) or swipe gesture (iOS)

### Screen Transitions
- Push/pop for drill-down navigation
- Modal sheets for settings/dialogs
- Fade transitions for tab switching

### Gestures
- Swipe back (iOS standard)
- Swipe between pages (launcher)
- Pull to refresh (where applicable)
- Long-press for context actions

## Configuration Screen (Mobile)

### Layout
- **List View**: Scrollable list of configuration sections
- **Section Headers**: Clear visual hierarchy
- **Detail View**: Full-screen for each section's settings

### Sections
- General
- Identity & Security
- Server & Pairing
- Applications
- Theme & Appearance
- Advanced

### Interaction
- Tap section to open detail view
- Top navigation bar with back button and title
- Save button (top-right or floating action button)
- Confirmation for destructive actions

## Mobile-Specific Features

### Pull to Refresh
- Launcher: Refresh app list
- App list: Check for updates
- Standard platform behavior

### Haptic Feedback
- iOS: UIImpactFeedbackGenerator
- Android: Vibration API
- On tap, long-press, drag events

### Safe Area Insets
- Respect notch/island (iPhone)
- Respect system bars
- Avoid overlapping interactive elements

### Adaptive Layout
- Portrait: Primary orientation
- Landscape: Adjusted grid (more columns)
- Tablet: Larger icons, more content

## Touch Interactions

### Touch Targets
- Minimum size: 48x48 dp/pt
- Adequate spacing between targets
- Clear visual feedback on touch

### Long-Press
- 500ms threshold for reorder mode
- Visual feedback after 250ms
- Cancel if drag starts before threshold

### Drag and Drop
- Pick up animation on drag start
- Drop preview (shadow/outline)
- Snap animation on drop
- Cancel gesture (drag out of bounds)

## Visual Design

### Color Scheme

**Light Mode**:
- Background: #FFFFFF
- Card background: #F5F5F5
- Text: #000000
- Accent: iOS Blue (#007AFF) or Android Green (#4CAF50)

**Dark Mode**:
- Background: #000000 (iOS) or #121212 (Android)
- Card background: #1C1C1E (iOS) or #1E1E1E (Android)
- Text: #FFFFFF
- Accent: iOS Blue (#0A84FF) or Android Green (#81C784)

### Typography
- **iOS**: San Francisco
- **Android**: Roboto
- **Sizes**:
  - Body: 16sp/pt
  - Headings: 20-28sp/pt
  - Labels: 14sp/pt

### Spacing
- Consistent 8dp/pt grid
- Standard padding: 16dp/pt
- Card margins: 8-16dp/pt

## Platform-Specific Behavior

### iOS
- Swipe back gesture
- Pull to dismiss modals
- Navigation bar (44pt height)
- Tab bar (49pt height + safe area)
- System fonts and icons
- iOS standard animations
- Share sheet integration

### Android
- Back button handling
- Material Design 3 components
- Navigation bar (56dp height)
- Bottom navigation (56dp height)
- Floating action button
- Material animations
- Share intent integration

## Accessibility

### Screen Reader Support
- VoiceOver (iOS) compatible
- TalkBack (Android) compatible
- Descriptive labels
- Hint text for actions

### Dynamic Type/Font Scaling
- Respects system font size settings
- Layout adapts to larger text
- No text truncation

### High Contrast Mode
- Follows system settings
- Increased contrast ratios
- Clear visual boundaries

### Reduce Motion
- Respects system settings
- Simplified animations
- Instant transitions option

## Performance

### Startup
- Fast launch (< 2 seconds to launcher)
- Splash screen (platform standard)
- Progressive loading

### Scrolling
- 60 FPS (or 120 FPS on ProMotion displays)
- Smooth, responsive scrolling
- No janky animations

### Memory
- Efficient image loading
- Unload off-screen content
- Background tab suspension

### Battery
- Minimal background activity
- Efficient network usage
- Pause non-essential work when backgrounded

## Offline Support

### Network Indicators
- Online/offline status
- Sync indicators
- Queue for pending operations

### Graceful Degradation
- Local functionality available offline
- Clear messaging about network features
- Auto-retry when online

## Notifications (Post-MVP)

### Types
- App updates available
- Sync completed
- Server connection issues
- Payment requests

### Behavior
- Respect system notification settings
- Clear, actionable messages
- Deep links to relevant screen

## Error States

### Network Errors
- Clear error message
- Retry button
- Offline mode explanation

### App Load Errors
- Friendly error message
- Troubleshooting tips
- Contact support option

### Empty States
- Helpful illustrations
- Clear call-to-action
- Onboarding hints

## Pairing Flow (Mobile-Specific)

### QR Code Scanning
- Full-screen camera view
- Scan frame overlay
- Auto-detect and parse QR
- Manual entry fallback

### Pairing Progress
- Clear status indicators
- Progress spinner
- Connection attempt counter
- Cancel option

### Success/Failure
- Confirmation screen (success)
- Clear error message (failure)
- Retry option
- Manual 4-word entry fallback

## Future Enhancements

- Widget support (iOS 14+, Android)
- Shortcuts/Siri support (iOS)
- Quick settings tiles (Android)
- App clips/Instant apps
- Handoff between devices (iOS)
- Multi-window (iPad, Android tablets)
- Picture-in-picture
- Dark icon support (app icon tinting)
