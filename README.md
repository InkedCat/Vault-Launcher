# Vault Launcher

Vault Launcher is a cross-platform desktop **Minecraft** launcher built with Tauri, SvelteKit, and TypeScript. It provides a modern UI for launching and managing Vaults (Modded Minecraft Clients).

## Disclaimer

**Important:** Vault Launcher only supports premium accounts. It does not and will never support cracked accounts. This application is designed exclusively to work with legitimate, properly licensed accounts. Any modification of the application to allow cracked accounts is not my responsibility and will result in legal consequences.

## Features

- Modern, responsive UI built with SvelteKit and Tailwind CSS
- 3D visualization capabilities through Three.js and Threlte
- Currently Windows only (macOS and Linux support planned for future releases)
- Microsoft Authentication via deep linking
- Internationalization ready with Inlang/Paraglide
- Dark/light mode support

## Tech Stack

- **Frontend**: SvelteKit 5, TypeScript, Tailwind CSS, Bits UI
- **Backend**: Tauri 2, Rust
- **3D Rendering**: Three.js, Threlte
- **Styling**: Tailwind CSS with plugins (tailwindcss-animate, etc.)
- **Package Management**: PNPM

## Development

### Prerequisites

- [Node.js](https://nodejs.org/en/) (v18 or later)
- [PNPM](https://pnpm.io/) (v10.4.1 or later)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- OS-specific Tauri dependencies (see [Tauri setup docs](https://tauri.app/start/prerequisites/#system-dependencies))

### Setup

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/vault-launcher.git
   cd vault-launcher
   ```

2. Install dependencies:

   ```bash
   pnpm install
   ```

3. Start the development server:

   ```bash
   pnpm dev
   ```

4. Launch the Tauri application in development mode:
   ```bash
   pnpm tauri dev
   ```

### Available Scripts

- `pnpm dev` - Start SvelteKit development server
- `pnpm build` - Build the SvelteKit application
- `pnpm tauri dev` - Run the Tauri application in development mode
- `pnpm tauri build` - Build the Tauri application for production
- `pnpm check` - Run type checking
- `pnpm format` - Format code with Prettier
- `pnpm lint` - Lint code
- `pnpm check-licenses` - Check licenses of dependencies
- `pnpm machine-translate` - Run machine translation for internationalization

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/)
- [Svelte for VS Code](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
- [Tauri for VS Code](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Building for Production

To build the application for production:

```bash
pnpm tauri build
```

This will create platform-specific installers in the `src-tauri/target/release/bundle` directory.

## License

This project is licensed under the GPL-3.0 License - see the LICENSE file for details.
