/* tslint:disable */
/* eslint-disable */
export function init(): void;
export function get_emc_standard(standard_name: string, emc_class: string, _interface?: string | null): any;
export function list_available_standards(): any;
export function list_standard_classes(standard_name: string): any;
export function calculate_emc_limit(standard_json: string, frequency: number): any;
export function check_emc_compliance(standard_json: string, frequencies: Float64Array, amplitudes: Float64Array, measurement_type: string): any;
export function generate_emc_mask(standard_json: string, f_min: number, f_max: number, points_per_decade: number): any;
export function analyze_emc_statistics(standard_json: string, frequencies: Float64Array, amplitudes: Float64Array): any;
export function generate_adaptive_emc_mask(standard_json: string, f_min: number, f_max: number, target_points: number): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly init: () => void;
  readonly get_emc_standard: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number, number];
  readonly list_available_standards: () => [number, number, number];
  readonly list_standard_classes: (a: number, b: number) => [number, number, number];
  readonly calculate_emc_limit: (a: number, b: number, c: number) => [number, number, number];
  readonly check_emc_compliance: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => [number, number, number];
  readonly generate_emc_mask: (a: number, b: number, c: number, d: number, e: number) => [number, number, number];
  readonly analyze_emc_statistics: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number, number];
  readonly generate_adaptive_emc_mask: (a: number, b: number, c: number, d: number, e: number) => [number, number, number];
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_export_3: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
