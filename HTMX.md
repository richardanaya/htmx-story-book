# HTMX Usage in Storybuilder

This project uses HTMX (Hypertext Markup eXtensions) to create a dynamic, single-page application experience while maintaining server-side rendering. Here are the key ways HTMX is implemented:

## Core Setup
- HTMX is loaded via CDN in `templates/index.hbs`
- Used version: 2.0.4
- Added integrity hash for security

## Key Features Used

### 1. Progressive Enhancement
- All links and forms work without JavaScript
- HTMX enhances them with AJAX when available
- Example: Book navigation works with full page reloads or partial updates

### 2. Partial Page Updates
- Main content area is updated via HTMX swaps
- Example: When clicking a book link:
```hbs
<a href="/book/{{this.id}}" 
   hx-get="/book/{{this.id}}" 
   hx-target="main" 
   hx-swap="innerHTML">
```

### 3. Form Handling
- Login form uses HTMX for submission:
```hbs
<form hx-post="/login" 
      hx-swap="outerHTML" 
      hx-target="#login-section">
```
- Handles both success and error states without full page reload

### 4. Navigation
- Book page navigation preserves browser history:
```hbs
<a hx-get="/book/{{../book_id}}/page/{{this.target_page_id}}"
   hx-push-url="true">
```
- Updates URL without full page reload

### 5. Authentication Flow
- Login/logout uses HTMX for seamless transitions
- Login success triggers full page refresh via:
```rust
.header("HX-Refresh", "true")
```
- Ensures consistent state after auth changes

### 6. Targeted Updates
- Specific sections are updated rather than full pages
- Example: Book content updates just the book section:
```hbs
hx-target="#book-page"
hx-swap="innerHTML"
```

## Benefits Achieved
- Reduced JavaScript complexity
- Maintained server-side rendering benefits
- Fast, responsive UI
- Progressive enhancement
- Preserved browser history
- Accessible fallbacks

## Key HTMX Attributes Used
- `hx-get` - For GET requests
- `hx-post` - For form submissions
- `hx-target` - Specifies update target
- `hx-swap` - Controls content replacement
- `hx-push-url` - Manages browser history
- `hx-trigger` - Custom event handling

This approach provides a modern, dynamic user experience while maintaining simplicity and server-side control.
