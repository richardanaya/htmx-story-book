# Storybuilder Architecture

## Overview
The Storybuilder application is a web-based "Choose Your Own Adventure" story platform built using Rust with the Axum web framework. It follows a client-server architecture with server-side rendering using Handlebars templates and progressive enhancement via HTMX.

## Key Architectural Patterns

### 1. Layered Architecture
The code is organized into distinct layers:
- **Presentation Layer**: Handlebars templates and HTMX interactions
- **Application Layer**: Axum route handlers and services
- **Domain Layer**: Models representing core entities (User, Book, Page, Choice)
- **Infrastructure Layer**: Service implementations and utilities

### 2. Server-Side Rendering (SSR) with Progressive Enhancement
- Primary content rendering happens server-side using Handlebars templates
- HTMX is used for progressive enhancement of interactive elements
- Both full-page and partial rendering modes are supported

### 3. Service-Oriented Architecture
Core functionality is encapsulated in services:
- `AuthService`: Handles authentication and JWT management
- `BookService`: Manages the library of books and story content
- Services are thread-safe and shared across the application via Arc

### 4. RESTful Routing
- Routes follow REST conventions:
  - `/pages/book/{id}` for book resources
  - `/pages/book/{book_id}/page/{page_id}` for page resources
  - `/components/login` for authentication actions

### 5. Authentication Flow
- JWT-based authentication using cookies
- Authentication state is checked on protected routes
- Login/logout components handle the authentication UI

## Key Components

### Models
- `User`: Represents user credentials and JWT claims
- `Book`: Contains book metadata and page structure
- `Page`: Represents a story page with content and choices
- `Choice`: Links between pages with descriptive text

### Services
- `AuthService`: Manages user authentication and JWT tokens
- `BookService`: Provides access to the story library and page navigation

### Templates
- `index.hbs`: Main layout template
- `book_page.hbs`: Template for rendering story pages
- `login.hbs`: Login form component
- `logged_in.hbs`: User status component

### Routes
- Index routes: `/` - Main landing page
- Book routes: `/pages/book/{id}` - Story navigation
- Auth routes: `/components/login`, `/components/logout`

## Technical Stack
- **Web Framework**: Axum
- **Templating**: Handlebars
- **Interactive Elements**: HTMX
- **Authentication**: JWT
- **State Management**: Arc for shared state
- **Styling**: Custom CSS

## Data Flow
1. User requests a page
2. Route handler:
   - Checks authentication
   - Retrieves necessary data from services
   - Renders appropriate template
3. HTMX interactions trigger partial updates
4. Server responds with either full page or partial content

## Error Handling
- Authentication errors redirect to home page
- Missing resources return appropriate HTTP status codes
- Template rendering errors are handled with expect() (could be improved)

## Areas for Improvement
- Add proper error handling middleware
- Implement persistent storage for books and users
- Add rate limiting for authentication endpoints
- Implement CSRF protection
- Add logging middleware
