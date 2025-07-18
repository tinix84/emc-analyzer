import polars as pl
import numpy as np
import re
from pathlib import Path

def read_emc_txt_file(file_path, header_skip_pattern="End of Header"):
    """
    Legge file TXT EMC con header, converte unità e restituisce DataFrame Polars
    
    Parameters:
    - file_path: percorso al file TXT
    - header_skip_pattern: pattern per identificare fine header (default: "End of Header")
    
    Returns:
    - polars.DataFrame con colonne: freq_hz, peak_v, avg_v
    """
    try:
        # Leggi il file e trova dove finisce l'header
        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
            lines = f.readlines()
        
        # Trova la riga dopo l'header
        data_start_line = 0
        for i, line in enumerate(lines):
            if header_skip_pattern in line:
                data_start_line = i + 1
                break
        
        # Salta le righe vuote dopo l'header
        while data_start_line < len(lines) and lines[data_start_line].strip() == '':
            data_start_line += 1
        
        # Cerca e valida le unità di misura
        units_detected = {"freq": None, "peak": None, "avg": None}
        units_line = None
        
        for i in range(data_start_line, min(data_start_line + 10, len(lines))):
            line = lines[i].strip()
            if line:
                # Cerca pattern di unità
                if "[MHz]" in line or "[dB" in line or "Frequency" in line:
                    units_line = i
                    print(f"📏 Unità rilevate alla riga {i + 1}: {line}")
                    
                    # Estrai unità
                    if "[MHz]" in line:
                        units_detected["freq"] = "MHz"
                    if "[dBμV]" in line or "[dBuV]" in line:
                        units_detected["peak"] = "dBμV"
                        units_detected["avg"] = "dBμV"
                    elif "[dBV]" in line:
                        units_detected["peak"] = "dBV"
                        units_detected["avg"] = "dBV"
                    elif "[V]" in line:
                        units_detected["peak"] = "V"
                        units_detected["avg"] = "V"
                    
                    break
        
        # Valida unità rilevate
        if units_detected["freq"] != "MHz":
            print("⚠️  Attenzione: frequenza non in MHz")
        if units_detected["peak"] not in ["dBμV", "dBuV"]:
            print("⚠️  Attenzione: peak non in dBμV")
        if units_detected["avg"] not in ["dBμV", "dBuV"]:
            print("⚠️  Attenzione: avg non in dBμV")
        
        print(f"✅ Unità rilevate: Freq={units_detected['freq']}, Peak={units_detected['peak']}, Avg={units_detected['avg']}")
        
        # Trova inizio dati numerici (dopo le unità)
        if units_line is not None:
            data_start_line = units_line + 1
        
        # Salta eventuali righe di info aggiuntive (non numeriche)
        while data_start_line < len(lines):
            line = lines[data_start_line].strip()
            if line and re.match(r'^\s*[0-9]+\.', line):
                break
            data_start_line += 1
        
        print(f"📄 Inizio dati rilevato alla riga {data_start_line + 1}")
        
        # Estrai i dati numerici
        data_lines = []
        for i in range(data_start_line, len(lines)):
            line = lines[i].strip()
            if line:
                # Usa regex per estrarre numeri da riga con tab/spazi
                # Formato tipico: "0.222     	      48.97	      35.74"
                numbers = re.findall(r'[0-9]+\.?[0-9]*', line)
                if len(numbers) >= 3:
                    try:
                        freq_mhz = float(numbers[0])
                        peak_dbuv = float(numbers[1]) 
                        avg_dbuv = float(numbers[2])
                        data_lines.append([freq_mhz, peak_dbuv, avg_dbuv])
                    except ValueError:
                        continue
        
        print(f"📊 Estratte {len(data_lines)} righe di dati")
        
        if not data_lines:
            raise ValueError("Nessun dato numerico trovato nel file")
        
        # Converti in DataFrame Polars
        df = pl.DataFrame(
            data_lines,
            schema=["freq_mhz", "peak_dbuv", "avg_dbuv"]
        )
        
        # Conversioni di unità basate su unità rilevate
        conversion_expressions = []
        
        # Frequenza: MHz → Hz (sempre)
        if units_detected["freq"] == "MHz":
            conversion_expressions.append((pl.col("freq_mhz") * 1e6).alias("freq_hz"))
        else:
            print("⚠️  Assumendo frequenza in MHz")
            conversion_expressions.append((pl.col("freq_mhz") * 1e6).alias("freq_hz"))
        
        # Tensione: dipende dalle unità rilevate
        if units_detected["peak"] == "dBμV" or units_detected["peak"] == "dBuV":
            # dBμV → V: V = 10^((dBμV - 120) / 20)
            conversion_expressions.extend([
                (10 ** ((pl.col("peak_dbuv") - 120) / 20)).alias("peak_v"),
                (10 ** ((pl.col("avg_dbuv") - 120) / 20)).alias("avg_v")
            ])
        elif units_detected["peak"] == "dBV":
            # dBV → V: V = 10^(dBV / 20)
            conversion_expressions.extend([
                (10 ** (pl.col("peak_dbuv") / 20)).alias("peak_v"),
                (10 ** (pl.col("avg_dbuv") / 20)).alias("avg_v")
            ])
        elif units_detected["peak"] == "V":
            # Già in V
            conversion_expressions.extend([
                pl.col("peak_dbuv").alias("peak_v"),
                pl.col("avg_dbuv").alias("avg_v")
            ])
        else:
            print("⚠️  Assumendo dBμV per le tensioni")
            conversion_expressions.extend([
                (10 ** ((pl.col("peak_dbuv") - 120) / 20)).alias("peak_v"),
                (10 ** ((pl.col("avg_dbuv") - 120) / 20)).alias("avg_v")
            ])
        
        df = df.with_columns(conversion_expressions).select([
            "freq_hz", "peak_v", "avg_v"
        ])
        
        # Verifica unità
        freq_range = f"{df['freq_hz'].min()/1e6:.3f} - {df['freq_hz'].max()/1e6:.3f} MHz"
        peak_range = f"{df['peak_v'].min()*1e6:.1f} - {df['peak_v'].max()*1e6:.1f} μV"
        avg_range = f"{df['avg_v'].min()*1e6:.1f} - {df['avg_v'].max()*1e6:.1f} μV"
        
        print(f"✅ Conversioni completate:")
        print(f"   - Frequenza: {freq_range}")
        print(f"   - Peak: {peak_range}")
        print(f"   - Average: {avg_range}")
        print(f"   - Punti dati: {len(df)}")
        
        return df
        
    except Exception as e:
        print(f"❌ Errore lettura file {file_path}: {e}")
        return pl.DataFrame()

def convert_to_dbuv_for_masks(df):
    """
    Converte DataFrame da V a dBμV per compatibilità con maschere EMC
    
    Parameters:
    - df: DataFrame Polars con colonne freq_hz, peak_v, avg_v
    
    Returns:
    - DataFrame con colonne freq_mhz, peak_dbuv, avg_dbuv
    """
    return df.with_columns([
        # Frequenza: Hz → MHz
        (pl.col("freq_hz") / 1e6).alias("freq_mhz"),
        
        # Tensione: V → dBμV
        # Formula: dBμV = 20 * log10(V) + 120
        (20 * pl.col("peak_v").log10() + 120).alias("peak_dbuv"),
        (20 * pl.col("avg_v").log10() + 120).alias("avg_dbuv")
    ]).select([
        "freq_mhz", "peak_dbuv", "avg_dbuv"
    ])

def validate_emc_units(df):
    """
    Valida che le unità siano corrette
    """
    # Verifica range frequenza (dovrebbe essere in Hz)
    freq_min, freq_max = df['freq_hz'].min(), df['freq_hz'].max()
    if freq_max < 1e6:  # Se max < 1MHz, probabilmente è in MHz
        print("⚠️  Attenzione: frequenze sembrano in MHz, non Hz")
        return False
    
    # Verifica range tensione (dovrebbe essere in V)
    peak_min, peak_max = df['peak_v'].min(), df['peak_v'].max()
    if peak_min > 1e-3:  # Se min > 1mV, probabilmente è in unità sbagliate
        print("⚠️  Attenzione: tensioni sembrano in unità sbagliate")
        return False
    
    print("✅ Unità validate correttamente")
    return True

# Test con il file corrente
if __name__ == "__main__":
    # Test lettura file
    current_dir = Path(__file__).parent
    test_file = current_dir / "data" / "250128_OBC758_1_OP1_HVneg.TXT"
    if not test_file.exists():
        print(f"❌ File di test non trovato: {test_file}")
        exit(1)

    print("🔄 Test lettura file EMC...")
    df = read_emc_txt_file(test_file)
    
    if not df.is_empty():
        print(f"\n📋 Anteprima dati:")
        print(df.head())
        
        # Valida unità
        validate_emc_units(df)
        
        # Converti per compatibilità maschere
        df_dbuv = convert_to_dbuv_for_masks(df)
        print(f"\n📊 Dati convertiti per maschere:")
        print(df_dbuv.head())
    else:
        print("❌ Nessun dato caricato")
