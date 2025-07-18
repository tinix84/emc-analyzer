# 📊 EMC Spectrum Analysis Tools

Questa cartella contiene strumenti di analisi scientifica per spettri EMC, separati dalla web app principale.

## 🔬 Contenuti

- **`emc_spectrum_analysis.ipynb`** - Notebook Jupyter completo per analisi spettrale
- **`requirements.txt`** - Dipendenze Python per l'ambiente di analisi
- **`data/`** - Cartella per file di misura e risultati
- **`exports/`** - Cartella per risultati esportati

## 🚀 Quick Start

### 1. Setup Environment
```bash
# Crea ambiente virtuale
python -m venv emc-env
source emc-env/bin/activate  # Linux/Mac
# emc-env\Scripts\activate   # Windows

# Installa dipendenze
pip install -r requirements.txt
```

### 2. Avvia Jupyter
```bash
jupyter notebook emc_spectrum_analysis.ipynb
```

### 3. Carica i tuoi dati
- Posiziona file CSV/TXT nella cartella `data/`
- Usa le funzioni del notebook per caricare e analizzare

## 🔗 Integrazione con Web App

Il notebook usa gli stessi file JSON degli standard EMC della web app:
- `../public/emc-standards.json` - Standard EMC condivisi
- Stessi algoritmi di interpolazione logaritmica
- Compatibilità dei formati dati

## 📈 Funzionalità

### Analisi Spettrale
- Caricamento dati multi-formato
- Visualizzazione interattiva con Plotly
- Confronto multi-standard

### Peak Detection
- Algoritmi avanzati scipy
- Analisi conformità automatica
- Visualizzazione violazioni

### Dashboard Interattivo
- Widget dinamici per parametri
- Analisi in tempo reale
- Export risultati

## 💡 Esempi d'uso

```python
# Carica standard EMC
emc_standards = load_emc_standards('../public/emc-standards.json')

# Carica dati di misura
data = parse_measurement_data('data/measurement.csv')

# Analizza picchi
peaks = detect_peaks(data, prominence=5)
compliance = analyze_peaks_vs_limits(peaks, emc_standards, 'EN55032_ClassB')

# Visualizza risultati
plot_peaks_analysis(data, peaks, emc_standards, 'EN55032_ClassB')
```

## 🎯 Vantaggi

- **Rapid Prototyping**: Sviluppo veloce di algoritmi
- **Analisi Avanzata**: Funzionalità scientifiche complete
- **Separazione**: Non interferisce con la web app
- **Compatibilità**: Usa gli stessi standard EMC
