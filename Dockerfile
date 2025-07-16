# Dockerfile (per sviluppo locale)
FROM node:18-alpine

# Install Rust
RUN apk add --no-cache curl build-base
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install wasm-pack
RUN cargo install wasm-pack

WORKDIR /app

# Copy package files
COPY package*.json ./
RUN npm install

# Copy source code
COPY . .

# Build WASM and Nuxt
RUN chmod +x build.sh && ./build.sh

EXPOSE 3000

CMD ["npm", "run", "dev"]

# docker-compose.yml
version: '3.8'

services:
  emc-analyzer:
    build: .
    ports:
      - "3000:3000"
    volumes:
      - .:/app
      - /app/node_modules
      - /app/wasm/target
    environment:
      - NODE_ENV=development
    command: npm run dev

# GitHub Actions workflow
# .github/workflows/deploy.yml
name: Deploy to Vercel

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  deploy:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
        cache: 'npm'
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        target: wasm32-unknown-unknown
    
    - name: Install wasm-pack
      run: cargo install wasm-pack
    
    - name: Install dependencies
      run: npm install
    
    - name: Build WASM
      run: |
        cd wasm
        wasm-pack build --target web --out-dir ../public/wasm
        cd ..
    
    - name: Build Nuxt
      run: npm run build
    
    - name: Deploy to Vercel
      uses: amondnet/vercel-action@v20
      with:
        vercel-token: ${{ secrets.VERCEL_TOKEN }}
        vercel-org-id: ${{ secrets.ORG_ID }}
        vercel-project-id: ${{ secrets.PROJECT_ID }}
        vercel-args: '--prod'
