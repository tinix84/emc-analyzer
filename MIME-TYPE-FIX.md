# MIME Type Error Fix

## Problem
The error "Failed to load module script: Expected a JavaScript-or-Wasm module script but the server responded with a MIME type of 'text/html'" occurs when:

1. The server serves HTML (usually the SPA fallback page) instead of JavaScript modules
2. The routing configuration doesn't properly distinguish between static assets and SPA routes
3. The base path configuration is incorrect

## Solution

### 1. Updated `vercel.json`
```json
{
  "buildCommand": "npm run build",
  "outputDirectory": "dist",
  "installCommand": "npm install",
  "framework": null,
  "headers": [
    {
      "source": "/assets/(.*)",
      "headers": [
        {
          "key": "Cache-Control",
          "value": "public, max-age=31536000, immutable"
        }
      ]
    }
  ],
  "rewrites": [
    {
      "source": "/assets/(.*)",
      "destination": "/assets/$1"
    },
    {
      "source": "/((?!assets).*)",
      "destination": "/index.html"
    }
  ]
}
```

**Key Changes:**
- Added proper asset handling for `/assets/` directory
- Used negative lookahead regex `((?!assets).*)` to exclude assets from SPA routing
- Added cache headers for better performance

### 2. Fixed `vite.config.ts`
```typescript
base: '/'
```

**Key Changes:**
- Removed conditional base path that was causing routing issues
- Set consistent base path for all environments

### 3. Added Fallback Files

#### `public/_redirects` (for Netlify and similar)
```
/*    /index.html   200
```

#### `public/.htaccess` (for Apache servers)
```
Options -MultiViews
RewriteEngine On
RewriteCond %{REQUEST_FILENAME} !-f
RewriteRule ^ index.html [QSA,L]
```

## Testing

1. **Local Testing:**
   ```bash
   npm run build
   cd dist
   python3 -m http.server 8080
   ```

2. **Production Deployment:**
   - Vercel: Automatic deployment with updated configuration
   - GitHub Pages: Uses existing workflow with proper routing

## Root Cause Analysis

The error occurred because:
1. All requests (including asset requests) were being routed to `index.html`
2. When the browser tried to load JavaScript modules, it received HTML instead
3. ES6 module loading has strict MIME type checking per HTML spec

## Prevention

- Always test SPA routing configuration with a local HTTP server
- Ensure asset directories are properly excluded from SPA fallback routing
- Use proper regex patterns for route matching
- Test both development and production builds

## References

- [ES6 Module MIME Type Requirements](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Modules)
- [Vercel SPA Configuration](https://vercel.com/docs/concepts/projects/project-configuration#rewrites)
- [Vite Base Path Configuration](https://vitejs.dev/config/shared-options.html#base)
