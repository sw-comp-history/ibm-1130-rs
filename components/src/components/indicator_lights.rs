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
