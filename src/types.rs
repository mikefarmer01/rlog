use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const DEMAND_PERIOD: &'static str = r#"
export class DemandPeriod {
    period: number;
    demand: number;
    demand_estimated: number;
    // TODO: What about the implementation (The constructor)?
}
"#;

#[wasm_bindgen(typescript_custom_section)]
const DEMAND_DATA: &'static str = r#"
export interface IDemandData {
    periods: [number],
    demands: [number],
    demands_estimated: [number]
    demandPeriods: DemandPeriod[]
}
"#;
