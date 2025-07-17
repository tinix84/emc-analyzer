# Vercel Deploy Instructions

## ⚠️ Problema WASM su Vercel

Vercel ha limitazioni con la compilazione Rust/WASM che causano errori di permessi e path.

## ✅ Soluzione Semplice

### Opzione 1: Solo Vue (Raccomandato per Vercel)
```json
// vercel.json
{
  "buildCommand": "npm run build",
  "outputDirectory": "dist",
  "installCommand": "npm install",
  "framework": null
}
```

### Opzione 2: Pre-compiled WASM
1. Compila WASM localmente: `npm run build:wasm`
2. Commit i file in `public/wasm/`
3. Deploy senza ricompilazione Rust

### Opzione 3: GitHub Pages (Supporta WASM)
- Usa il workflow `github-pages.yml`
- Compilazione Rust funziona su GitHub Actions
- URL: `https://tinix84.github.io/nuxtjs-boilerplate/`

## 🚀 Deploy Raccomandato

**Per Vercel:** Usa solo Vue senza WASM
**Per funzioni WASM:** Usa GitHub Pages

## 🛠️ Alternative

1. **Netlify** - Supporta meglio Rust/WASM
2. **GitHub Pages** - Build actions complete
3. **Railway** - Supporto Docker completo
