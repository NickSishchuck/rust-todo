# Minimalist TODO App

A responsive TODO application built with Rust and Yew WebAssembly framework. Features a clean, minimalist design with light/dark theme support and persistent local storage.

![Peek 2025-06-04 22-16](https://github.com/user-attachments/assets/ae98dc45-faa5-441f-9755-858945907c29)

## 🚀 Getting Started

### Prerequisites

- Rust
- Trunk - WebAssembly build tool for Rust

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/NickSishchuck/rust-todo
   cd todo-app
   ```

2. **Install Trunk** (if not already installed)
   ```bash
   cargo install trunk
   ```

3. **Install dependencies**
   ```bash
   cargo check
   ```

### Running the Development Server

```bash
trunk serve
```

This will:
- Compile the Rust code to WebAssembly
- Bundle all CSS files
- Start a development server at `http://127.0.0.1:8080`
- Enable hot reloading for development

Open your browser and navigate to the displayed URL to see the app in action.

### Building

```bash
trunk build --release
```

This creates a `dist/` directory with optimized files ready for deployment to any static hosting service.

## 🏗️ Project Structure

```
todo-app/
├── src/
│   ├── components/          # Reusable UI components
│   │   ├── app.rs          # Main application component
│   │   ├── header.rs       # App header with logo and controls
│   │   ├── todo_input.rs   # Task input form
│   │   ├── todo_list.rs    # Task list container
│   │   ├── todo_item.rs    # Individual task component
│   │   ├── filter_tabs.rs  # Filter navigation tabs
│   │   ├── theme_switch.rs # Theme toggle switch
│   │   └── mod.rs          # Component module exports
│   ├── hooks/              # Custom React-like hooks
│   │   ├── use_theme.rs    # Theme management hook
│   │   └── mod.rs          # Hook module exports
│   ├── models/             # Data models and types
│   │   └── mod.rs          # Todo and Filter types
│   └── main.rs             # Application entry point
├── styles/                 # CSS stylesheets
│   ├── base.css           # Reset and base styles
│   ├── theme.css          # Light/dark theme variables
│   ├── components.css     # Component-specific styles
│   └── animations.css     # Animations and transitions
├── index.html             # HTML template
└── Cargo.toml            # Rust dependencies and metadata
```

## 🛠️ Technologies Used

- **[Rust](https://www.rust-lang.org/)**
- **[Yew](https://yew.rs/)** - Modern Rust framework for creating multi-threaded front-end web apps
- **[WebAssembly](https://webassembly.org/)** - Binary instruction format for web browsers
- **[Trunk](https://trunkrs.dev/)** - Build tool and dev server for Rust/Wasm applications
- **CSS Custom Properties** - For dynamic theming
- **localStorage API** - For persistent data storage

### Key Dependencies

- `yew` - The main framework for building the UI
- `web-sys` - Bindings for Web APIs
- `wasm-bindgen` - Facilitating high-level interactions between Wasm and JavaScript
- `serde` & `serde_json` - Serialization for localStorage
- `uuid` - Generating unique identifiers for tasks
- `gloo` - Toolkit for working with browser APIs


### Styling

The app uses a systematic approach to styling:

- **base.css**: Reset, typography, and base layout
- **theme.css**: CSS custom properties for light/dark themes
- **components.css**: Component-specific styles
- **animations.css**: Transitions and keyframe animations

### State Management

State is managed using Yew's built-in hooks:
- `use_state` for component-level state
- `use_effect_with` for side effects and lifecycle management
- Custom `use_theme` hook for theme management

## 📱 Browser Support

Works in all modern browsers that support WebAssembly:
- Chrome/Chromium 57+
- Firefox 52+
- Safari 11+
- Edge 16+

(God I Hate Frontend Development)
