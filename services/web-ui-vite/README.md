# FlowMason Web UI (React/Vite)

Modern React frontend built with Vite, TypeScript, and Tailwind CSS.

## Prerequisites

- Node.js 18+ and npm

## Development

Install dependencies:

```bash
npm install
```

Start the development server:

```bash
npm run dev
```

The UI will be available at **http://localhost:8080**

The dev server automatically:
- Hot module replacement (instant updates)
- Fast refresh for React components
- TypeScript type checking
- Proxy API requests to `http://localhost:3000`

## Production Build

Build the optimized production bundle:

```bash
npm run build
```

The built files will be in `dist/` directory.

## Integration with API Server

The API server (`services/api`) is configured to serve static files from `services/web-ui-vite/dist/` when they exist.

1. Build the React app: `cd services/web-ui-vite && npm run build`
2. Start the API server: `cd services/api && cargo run`
3. Access the UI at **http://localhost:3000**

## Project Structure

- `src/` - Source code
  - `main.tsx` - Entry point
  - `App.tsx` - Main app component
  - `router.tsx` - Route definitions
  - `api/` - API client
  - `components/` - Reusable components
    - `layouts/` - Layout components
    - `ui/` - UI components
  - `pages/` - Page components
- `index.html` - HTML template
- `dist/` - Build output (generated)

## Notes

- The UI uses Tailwind CSS for styling
- API calls are made to `/api/v1/*` endpoints
- All routes are client-side routed using React Router
- TypeScript provides type safety throughout the application
