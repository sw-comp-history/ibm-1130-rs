// Sixteen Bit Panel Component
//
// Displays 16 toggle switches organized in 4 groups of 4 (nibbles).
// Shows binary, hexadecimal, and decimal values.
// Ported from toggle-nixie SixteenBitView React component.

use yew::prelude::*;
use crate::components::toggle_switch::ToggleSwitch;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum PanelMode {
    #[default]
    Interactive,
    AutoIncrement,
    AutoDecrement,
    Display, // Read-only display mode
}

#[derive(Properties, PartialEq)]
pub struct SixteenBitPanelProps {
    /// Current 16-bit value
    #[prop_or(0)]
    pub value: u16,
    /// Callback when value changes
    #[prop_or_default]
    pub on_change: Callback<u16>,
    /// Operating mode
    #[prop_or_default]
    pub mode: PanelMode,
    /// Optional label for the panel
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// Show hex/decimal display
    #[prop_or(true)]
    pub show_value_display: bool,
}

#[function_component(SixteenBitPanel)]
pub fn sixteen_bit_panel(props: &SixteenBitPanelProps) -> Html {
    let value = use_state(|| props.value);

    // Sync with external value changes
    {
        let value = value.clone();
        let prop_value = props.value;
        use_effect_with(prop_value, move |&new_val| {
            value.set(new_val);
            || ()
        });
    }

    // Auto increment/decrement effect
    {
        let value = value.clone();
        let on_change = props.on_change.clone();
        let mode = props.mode;
        use_effect_with(mode, move |&mode| {
            let interval: Option<gloo::timers::callback::Interval> =
                if mode == PanelMode::AutoIncrement || mode == PanelMode::AutoDecrement {
                    Some(gloo::timers::callback::Interval::new(500, move || {
                        let current = *value;
                        let next = match mode {
                            PanelMode::AutoIncrement => current.wrapping_add(1),
                            PanelMode::AutoDecrement => current.wrapping_sub(1),
                            _ => current,
                        };
                        value.set(next);
                        on_change.emit(next);
                    }))
                } else {
                    None
                };
            // Return cleanup function that drops the interval
            move || drop(interval)
        });
    }

    let toggle_bit = {
        let value = value.clone();
        let on_change = props.on_change.clone();
        let mode = props.mode;
        Callback::from(move |bit: u8| {
            if mode == PanelMode::Interactive {
                let current = *value;
                let new_value = current ^ (1 << (15 - bit));
                value.set(new_value);
                on_change.emit(new_value);
            }
        })
    };

    let hex_string = format!("{:04X}", *value);
    let decimal_string = format!("{}", *value);

    html! {
        <div class="sixteen-bit-panel">
            if let Some(label) = &props.label {
                <div class="panel-label">{label.clone()}</div>
            }

            // Value display (hex and decimal)
            if props.show_value_display {
                <div class="value-display">
                    <span class="hex-value">{"0x"}{&hex_string}</span>
                    <span class="decimal-value">{" = "}{&decimal_string}<sub>{"10"}</sub></span>
                </div>
            }

            // Toggle switches section with nibble dividers
            <div class="switches-section">
                <div class="switches-row">
                    // Leading divider line (left of switch 0)
                    <div class="nibble-divider" />
                    // 4 nibbles (groups of 4 switches each) with dividers between
                    { for (0..4).map(|nibble_idx| {
                        html! {
                            <>
                                <div class="nibble-switches">
                                    { for (0..4).map(|bit_in_nibble| {
                                        let bit_position = nibble_idx * 4 + bit_in_nibble;
                                        let is_on = (*value >> (15 - bit_position)) & 1 == 1;
                                        let weight = 8 >> bit_in_nibble; // 8, 4, 2, 1

                                        let toggle_bit = toggle_bit.clone();
                                        let on_toggle = Callback::from(move |_| {
                                            toggle_bit.emit(bit_position);
                                        });

                                        html! {
                                            <div class="switch-with-label">
                                                <div class="switch-number">{bit_position}</div>
                                                <ToggleSwitch
                                                    value={weight}
                                                    is_on={is_on}
                                                    on_toggle={on_toggle}
                                                    disabled={props.mode != PanelMode::Interactive}
                                                />
                                            </div>
                                        }
                                    })}
                                </div>
                                // Divider after each nibble
                                <div class="nibble-divider" />
                            </>
                        }
                    })}
                </div>
            </div>
        </div>
    }
}

/// Get a single bit from a u16 value
pub fn get_bit(value: u16, bit: u8) -> bool {
    (value >> (15 - bit)) & 1 == 1
}

/// Set a single bit in a u16 value
pub fn set_bit(value: u16, bit: u8, on: bool) -> u16 {
    if on {
        value | (1 << (15 - bit))
    } else {
        value & !(1 << (15 - bit))
    }
}

/// Toggle a single bit in a u16 value
pub fn toggle_bit(value: u16, bit: u8) -> u16 {
    value ^ (1 << (15 - bit))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bit() {
        assert!(get_bit(0x8000, 0)); // MSB set
        assert!(!get_bit(0x8000, 1));
        assert!(get_bit(0x0001, 15)); // LSB set
        assert!(!get_bit(0x0001, 0));
    }

    #[test]
    fn test_set_bit() {
        assert_eq!(set_bit(0x0000, 0, true), 0x8000);
        assert_eq!(set_bit(0x8000, 0, false), 0x0000);
        assert_eq!(set_bit(0x0000, 15, true), 0x0001);
    }

    #[test]
    fn test_toggle_bit() {
        assert_eq!(toggle_bit(0x0000, 0), 0x8000);
        assert_eq!(toggle_bit(0x8000, 0), 0x0000);
        assert_eq!(toggle_bit(0xFFFF, 8), 0xFF7F);
    }

    #[test]
    fn test_panel_mode_default() {
        assert_eq!(PanelMode::default(), PanelMode::Interactive);
    }
}
