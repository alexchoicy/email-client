# Tauri + SvelteKit + TypeScript

This template should help get you started developing with Tauri, SvelteKit and TypeScript in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

# Setup

## 1. Install Dependencies

```bash
pnpm install
```

## 2. Environment Configuration

Create a `.env` file in the `src-tauri` directory (next to `Cargo.toml`)

```bash
# src-tauri/.env
GOOGLE_CLIENT_ID=your_google_client_id_here
GOOGLE_CLIENT_SECRET=your_google_client_secret_here
GOOGLE_TOKEN_URL=https://oauth2.googleapis.com/token
GOOGLE_AUTH_URL=https://accounts.google.com/o/oauth2/v2/auth
```

## 3. Running the Application

```bash
# Development mode
pnpm run tauri dev

# Build for production
pnpm run tauri build
```