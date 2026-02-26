// Power Switch Component
//
// An ON/OFF sliding power switch with a red/orange background.
// Ported from knob-lamps PowerSwitch React component.

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PowerSwitchProps {
    /// Current state of the power switch
    pub is_on: bool,
    /// Callback when switch is toggled
    pub on_toggle: Callback<()>,
}

#[function_component(PowerSwitch)]
pub fn power_switch(props: &PowerSwitchProps) -> Html {
    let onclick = {
        let on_toggle = props.on_toggle.clone();
        Callback::from(move |_: MouseEvent| {
            on_toggle.emit(());
        })
    };

    // OFF: slider at bottom (down), handle above
    // ON: slider at top (up), handle below
    let slider_y = if props.is_on { 12 } else { 55 };
    let handle_y = if props.is_on { 52 } else { 15 };  // Handle below or above slider

    html! {
        <div class="power-switch-container">
            <svg
                viewBox="0 0 100 100"
                class="power-switch-svg"
                onclick={onclick}
            >
                // Red/orange background square
                <rect
                    x="5"
                    y="5"
                    width="90"
                    height="90"
                    rx="6"
                    fill="#c94a3a"
                    stroke="#a03020"
                    stroke-width="2"
                />

                // Dark vertical handle/bar (prominent)
                <rect
                    x="38"
                    y={handle_y.to_string()}
                    width="24"
                    height="38"
                    rx="3"
                    fill="#1a1a1a"
                />

                // White sliding toggle bar (large and prominent)
                <rect
                    x="8"
                    y={slider_y.to_string()}
                    width="84"
                    height="38"
                    rx="4"
                    fill="#f8f8f8"
                    stroke="#c0c0c0"
                    stroke-width="1"
                />

                // ON text (left side of slider)
                <text
                    x="14"
                    y={(slider_y + 16).to_string()}
                    font-size="11"
                    font-weight="bold"
                    fill="#2d3748"
                    font-family="Arial, sans-serif"
                >
                    {"ON"}
                </text>

                // POWER text (center of slider)
                <text
                    x="50"
                    y={(slider_y + 26).to_string()}
                    font-size="14"
                    font-weight="bold"
                    fill="#2d3748"
                    font-family="Arial, sans-serif"
                    text-anchor="middle"
                >
                    {"POWER"}
                </text>

                // OFF text - right of slider when ON, below slider when OFF
                if props.is_on {
                    <text
                        x="86"
                        y={(slider_y + 16).to_string()}
                        font-size="11"
                        font-weight="bold"
                        fill="#2d3748"
                        font-family="Arial, sans-serif"
                        text-anchor="end"
                    >
                        {"OFF"}
                    </text>
                } else {
                    <text
                        x="50"
                        y={(slider_y + 48).to_string()}
                        font-size="11"
                        font-weight="bold"
                        fill="#f0f0f0"
                        font-family="Arial, sans-serif"
                        text-anchor="middle"
                    >
                        {"OFF"}
                    </text>
                }
            </svg>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_switch_props() {
        let props = PowerSwitchProps {
            is_on: true,
            on_toggle: Callback::noop(),
        };
        assert!(props.is_on);
    }
}
