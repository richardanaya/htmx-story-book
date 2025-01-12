# Architecture Interview: Building a Modern Web App with Classic Tools

Q: What inspired the overall architecture of this project?

A: I wanted to create something that felt modern but avoided unnecessary complexity. I took inspiration from frameworks like NextJS in terms of how they organize content and pages, but wanted to strip things back to basics. The goal was to make it obvious where to find things and how to modify them.

Q: Can you elaborate on your choice of HTMX over more popular frontend frameworks?

A: HTMX represents a return to simplicity in web development. Instead of building complex client-side applications, HTMX lets us handle most interactions server-side while still providing a smooth user experience. It's just HTML attributes - no need to write JavaScript, manage state, or deal with complex build systems. The learning curve is minimal, and it works beautifully with server-side rendering.

Q: The project has a very organized structure around pages and components. What drove that decision?

A: I wanted clear separation between different types of content while keeping related files together. Each page or component has its own directory containing both the Rust handlers and Handlebars templates. This makes it really easy to find everything related to a specific feature. It's similar to how NextJS organizes pages, but adapted for a Rust/Handlebars environment.

Q: Speaking of Handlebars, why choose it over other templating options?

A: Handlebars is battle-tested technology that just works. While there are Rust-specific options like Askama that offer compile-time template checking, they often introduce complexity through special macros and can cause confusing compilation errors. Handlebars is simple, well-documented, and doesn't try to be too clever. It's also familiar to developers coming from other ecosystems.

Q: The CSS approach is notably minimalist. What's the philosophy there?

A: I wanted to avoid the complexity of CSS frameworks while still maintaining a modern, maintainable approach. The strategy was to rely heavily on semantic HTML elements and their default styling where possible. For custom needs, I used Open Props which provides low-level design tokens - kind of like CSS custom properties on steroids. This gives us the benefits of a design system without the overhead of a full framework.

Q: How does the authentication system reflect this philosophy of simplicity?

A: The auth system uses JWTs stored in cookies - a proven approach that's well-understood. The handlers are straightforward and the login flow is handled entirely through HTMX. No need for complex OAuth flows or session management when a simple token-based approach meets our needs.

Q: What would you say to developers who might consider this approach "too simple"?

A: Simplicity is a feature, not a bug. By choosing stable, well-understood technologies and organizing them thoughtfully, we've created something that's easy to understand, maintain, and extend. There's no need for complexity unless it truly adds value. This codebase proves you can build modern, interactive web applications without reaching for the latest framework or tool.

Q: Any final thoughts on the architecture?

A: This project demonstrates that you can create a well-structured, maintainable web application using simple, proven technologies. The key is thoughtful organization and careful selection of tools that solve real problems without introducing unnecessary complexity. Sometimes the best solution is the simplest one that gets the job done.
