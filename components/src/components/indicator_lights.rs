// Indicator Lights Component
//
// Displays a row of 16 indicator lights representing a 16-bit register value.
// Features warm white backlit indicators with glow effects.
// Ported from knob-lamps IndicatorDisplay React component.

use yew::prelude::*;

/// Represents the state of an indicator light
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IndicatorState {
    /// Light is off
    Off,
    /// Light is on (specific bit position)
    On,
    /// All lights on (lamp test mode)
    AllOn,
}

#[derive(Properties, PartialEq)]
pub struct IndicatorLightsProps {
    /// The 16-bit value to display
    pub value: u16,
    /// Row label (e.g., "ACCUMULATOR")
    pub label: AttrValue,
    /// Whether all lights should be on (lamp test mode)
    #[prop_or(false)]
    pub lamp_test: bool,
    /// Whether the display is powered on
    #[prop_or(true)]
    pub power_on: bool,
}

#[function_component(IndicatorLights)]
pub fn indicator_lights(props: &IndicatorLightsProps) -> Html {
    let show_lights = props.power_on;

    html! {
        <div class="indicator-row">
            <div class="row-label">
                { for props.label.split('\n').map(|line| {
                    html! { <div>{line}</div> }
                })}
            </div>
            <div class="indicators">
                { for (0..16).map(|bit| {
                    let is_lit = show_lights && (props.lamp_test || ((props.value >> (15 - bit)) & 1 == 1));
                    let class = if is_lit { "indicator lit" } else { "indicator unlit" };

                    // Show bit position as hex digit (0-F)
                    let label = format!("{:X}", bit);

                    html! {
                        <div class={class}>
                            {label}
                        </div>
                    }
                })}
            </div>
        </div>
    }
}

/// Register Display Component
///
/// Displays 6 rows of indicator lights for IBM 1130 registers.
#[derive(Properties, PartialEq)]
pub struct RegisterDisplayProps {
    /// Instruction Address Register (IAR)
    pub iar: u16,
    /// Storage Address Register (SAR)
    pub sar: u16,
    /// Storage Buffer Register (SBR)
    pub sbr: u16,
    /// Arithmetic Factor Register (AFR)
    pub afr: u16,
    /// Accumulator (ACC)
    pub acc: u16,
    /// Accumulator Extension (EXT)
    pub ext: u16,
    /// Lamp test mode
    #[prop_or(false)]
    pub lamp_test: bool,
    /// Power state
    #[prop_or(true)]
    pub power_on: bool,
}

#[function_component(RegisterDisplay)]
pub fn register_display(props: &RegisterDisplayProps) -> Html {
    html! {
        <div class="indicator-display">
            <IndicatorLights
                value={props.iar}
                label="INSTRUCTION\nADDRESS"
                lamp_test={props.lamp_test}
                power_on={props.power_on}
            />
            <IndicatorLights
                value={props.sar}
                label="STORAGE\nADDRESS"
                lamp_test={props.lamp_test}
                power_on={props.power_on}
            />
            <IndicatorLights
                value={props.sbr}
                label="STORAGE\nBUFFER"
                lamp_test={props.lamp_test}
                power_on={props.power_on}
            />
            <IndicatorLights
                value={props.afr}
                label="ARITHMETIC\nFACTOR"
                lamp_test={props.lamp_test}
                power_on={props.power_on}
            />
            <IndicatorLights
                value={props.acc}
                label="ACCUMULATOR"
                lamp_test={props.lamp_test}
                power_on={props.power_on}
            />
            <IndicatorLights
                value={props.ext}
                label="ACCUMULATOR\nEXTENSION"
                lamp_test={props.lamp_test}
                power_on={props.power_on}
            />
        </div>
    }
}

/// Control Panel Display Component (Right Side)
///
/// Displays the vertical control indicators:
/// - OP Code (5 bits)
/// - Format/Tag (F, T0, T1)
/// - Cycle indicators (T0-T7)
/// - Status indicators (W, R, IA)
#[derive(Properties, PartialEq)]
pub struct ControlDisplayProps {
    /// Operation code (5 bits)
    #[prop_or(0)]
    pub op_code: u8,
    /// Format bit (long instruction)
    #[prop_or(false)]
    pub format: bool,
    /// Tag bits (index register selection)
    #[prop_or(0)]
    pub tag: u8,
    /// Cycle/timing indicator (0-7)
    #[prop_or(0)]
    pub cycle: u8,
    /// Wait state
    #[prop_or(false)]
    pub wait: bool,
    /// Run state
    #[prop_or(false)]
    pub run: bool,
    /// Indirect addressing
    #[prop_or(false)]
    pub indirect: bool,
    /// Carry flag
    #[prop_or(false)]
    pub carry: bool,
    /// Overflow flag
    #[prop_or(false)]
    pub overflow: bool,
    /// Lamp test mode
    #[prop_or(false)]
    pub lamp_test: bool,
    /// Power state
    #[prop_or(true)]
    pub power_on: bool,
}

#[function_component(ControlDisplay)]
pub fn control_display(props: &ControlDisplayProps) -> Html {
    let show = props.power_on;

    html! {
        <div class="control-display">
            // OP Code column (5 bits)
            <div class="control-column">
                <div class="column-label">{"OP"}</div>
                { for (0..5).map(|bit| {
                    let is_lit = show && (props.lamp_test || ((props.op_code >> (4 - bit)) & 1 == 1));
                    let class = if is_lit { "control-indicator lit" } else { "control-indicator unlit" };
                    html! { <div class={class}>{bit}</div> }
                })}
            </div>

            // Tag/Format column
            <div class="control-column">
                <div class="column-label">{"TAG"}</div>
                <div class={classes!("control-indicator", (show && (props.lamp_test || props.format)).then_some("lit"))}>{"F"}</div>
                <div class={classes!("control-indicator", (show && (props.lamp_test || (props.tag & 2 != 0))).then_some("lit"))}>{"T0"}</div>
                <div class={classes!("control-indicator", (show && (props.lamp_test || (props.tag & 1 != 0))).then_some("lit"))}>{"T1"}</div>
                <div class="column-label">{"IA"}</div>
                <div class={classes!("control-indicator", (show && (props.lamp_test || props.indirect)).then_some("lit"))}>{"IND"}</div>
            </div>

            // Cycle column (T0-T7)
            <div class="control-column">
                <div class="column-label">{"CYC"}</div>
                { for (0..8).map(|t| {
                    let is_lit = show && (props.lamp_test || props.cycle == t);
                    let class = if is_lit { "control-indicator lit" } else { "control-indicator unlit" };
                    html! { <div class={class}>{format!("T{}", t)}</div> }
                })}
            </div>

            // Status column
            <div class="control-column">
                <div class="column-label">{"STS"}</div>
                <div class={classes!("control-indicator", (show && (props.lamp_test || props.wait)).then_some("lit"))}>{"W"}</div>
                <div class={classes!("control-indicator", (show && (props.lamp_test || props.run)).then_some("lit"))}>{"R"}</div>
                <div class={classes!("control-indicator", (show && (props.lamp_test || props.carry)).then_some("lit"))}>{"C"}</div>
                <div class={classes!("control-indicator", (show && (props.lamp_test || props.overflow)).then_some("lit"))}>{"V"}</div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indicator_state() {
        let off = IndicatorState::Off;
        let on = IndicatorState::On;
        assert_ne!(off, on);
    }

    #[test]
    fn test_bit_extraction() {
        let value: u16 = 0x8001; // MSB and LSB set

        // Check MSB (bit 0)
        assert_eq!((value >> 15) & 1, 1);

        // Check LSB (bit 15)
        assert_eq!((value >> 0) & 1, 1);

        // Check middle bit (bit 8)
        assert_eq!((value >> 7) & 1, 0);
    }
}
