# famfin Frontend

Household Finance Manager UI built with SvelteKit and Svelte 5.

## Setup

```bash
npm install
npm run dev
```

The frontend will be available at `http://localhost:5173` and proxies API calls to `http://localhost:3000`.

## Build

```bash
npm run build
npm run preview
```

## Features

- User authentication (login, create household)
- Dashboard with monthly spending overview
- Category breakdown with progress visualization
- Session management with localStorage persistence

## Structure

```
src/
  components/      # Reusable UI components
  lib/
    stores.ts      # Svelte stores and API utilities
    types.ts       # TypeScript types
  App.svelte       # Main app component with routing
  main.ts          # Application entry point
```
