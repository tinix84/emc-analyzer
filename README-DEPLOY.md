# 🔬 EMC Vue Analyzer

**Electromagnetic Compatibility Analysis Tool** with Vue.js and WebAssembly

## 🚀 Deploy Status

### ✅ **Deploy Scripts Corretti**

| Piattaforma | Workflow | Status |
|-------------|----------|--------|
| **Vercel** | `.github/workflows/deploy.yml` | ✅ Configurato |
| **GitHub Pages** | `.github/workflows/github-pages.yml` | ✅ Configurato |

### 📁 **File di Configurazione**

- **`vercel.json`** - ✅ Aggiornato per Vue + Vite
- **`vite.config.ts`** - ✅ Base path per GitHub Pages
- **`package.json`** - ✅ Script build corretti

## 🛠️ **Configurazione Deploy**

### **Vercel Deploy**

1. **Secrets richiesti** (GitHub Repository Settings → Secrets):
   ```
   VERCEL_TOKEN=your_vercel_token
   ORG_ID=your_vercel_org_id  
   PROJECT_ID=your_vercel_project_id
   ```

2. **Comando build**: `npm run build:wasm && npm run build`
3. **Output directory**: `dist`
4. **Framework**: `null` (Vue + Vite custom)

### **GitHub Pages Deploy**

1. **Abilitare Pages** nel repository settings
2. **Source**: GitHub Actions
3. **Auto-deploy** su push a `main`
4. **URL**: `https://tinix84.github.io/nuxtjs-boilerplate/`

## 📦 **Build Commands**

```bash
# Sviluppo
npm run dev

# Build produzione
npm run build

# Build con type checking
npm run build:check

# Build WASM
npm run build:wasm

# Preview build
npm run preview
```

## 🔧 **Struttura Deploy**

```
dist/                    # Build output
├── index.html          # Vue SPA entry
├── assets/             # CSS/JS chunks
├── *.html              # Demo pages
├── sample-*.txt/csv    # File esempio
└── wasm/               # WebAssembly modules
```

## ⚙️ **Environment Variables**

- **`NODE_ENV=production`** - Attiva base path GitHub Pages
- **`BASE_URL`** - Path base personalizzato (opzionale)

## 🎯 **Funzionalità**

- ✅ **Upload file** CSV/TXT/DAT
- ✅ **Standard EMC** EN55032, CISPR32
- ✅ **Grafico semi-log** interattivo
- ✅ **Storage persistente** localStorage
- ✅ **Export dati** PNG/JSON/CSV
- ✅ **WASM performance** Rust modules

## 🐛 **Troubleshooting**

### Build Fails
```bash
# Clear cache e reinstalla
rm -rf node_modules dist
npm install
npm run build
```

### WASM Issues
```bash
# Rebuild WASM modules
npm run build:wasm
```

### Type Errors
```bash
# Build senza type checking
npm run build
# Oppure con checking
npm run build:check
```

---

**📊 Ready for production deploy!** 🚀
