/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export const init: () => void;
export const get_emc_standard: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number, number];
export const list_available_standards: () => [number, number, number];
export const list_standard_classes: (a: number, b: number) => [number, number, number];
export const calculate_emc_limit: (a: number, b: number, c: number) => [number, number, number];
export const check_emc_compliance: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => [number, number, number];
export const generate_emc_mask: (a: number, b: number, c: number, d: number, e: number) => [number, number, number];
export const analyze_emc_statistics: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number, number];
export const generate_adaptive_emc_mask: (a: number, b: number, c: number, d: number, e: number) => [number, number, number];
export const __wbindgen_malloc: (a: number, b: number) => number;
export const __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
export const __wbindgen_free: (a: number, b: number, c: number) => void;
export const __wbindgen_export_3: WebAssembly.Table;
export const __externref_table_dealloc: (a: number) => void;
export const __wbindgen_start: () => void;
