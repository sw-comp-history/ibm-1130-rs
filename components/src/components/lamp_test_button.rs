// Lamp Test Switch Component
//
// A momentary switch that lights all indicator lamps when pressed.
// Similar to PowerSwitch but with white/gray background.

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LampTestButtonProps {
    /// Callback when button is pressed down
    pub on_press: Callback<()>,
    /// Callback when button is released
    pub on_release: Callback<()>,
    /// Whether the button is disabled
    #[prop_or(false)]
    pub disabled: bool,
}

#[function_component(LampTestButton)]
pub fn lamp_test_button(props: &LampTestButtonProps) -> Html {
    let is_pressed = use_state(|| false);

    let onmousedown = {
        let on_press = props.on_press.clone();
        let disabled = props.disabled;
        let is_pressed = is_pressed.clone();
        Callback::from(move |_: MouseEvent| {
            if !disabled {
                is_pressed.set(true);
                on_press.emit(());
            }
        })
    };

    let onmouseup = {
        let on_release = props.on_release.clone();
        let disabled = props.disabled;
        let is_pressed = is_pressed.clone();
        Callback::from(move |_: MouseEvent| {
            if !disabled {
                is_pressed.set(false);
                on_release.emit(());
            }
        })
    };

    let onmouseleave = {
        let on_release = props.on_release.clone();
        let disabled = props.disabled;
        let is_pressed = is_pressed.clone();
        Callback::from(move |_: MouseEvent| {
            if !disabled && *is_pressed {
                is_pressed.set(false);
                on_release.emit(());
            }
        })
    };

    // Touch events for mobile
    let ontouchstart = {
        let on_press = props.on_press.clone();
        let disabled = props.disabled;
        let is_pressed = is_pressed.clone();
        Callback::from(move |_: TouchEvent| {
            if !disabled {
                is_pressed.set(true);
                on_press.emit(());
            }
        })
    };

    let ontouchend = {
        let on_release = props.on_release.clone();
        let disabled = props.disabled;
        let is_pressed = is_pressed.clone();
        Callback::from(move |_: TouchEvent| {
            if !disabled {
                is_pressed.set(false);
                on_release.emit(());
            }
        })
    };

    // When pressed, slider moves up (like ON state)
    // When released, slider is down (like OFF state)
    let slider_y = if *is_pressed { 12 } else { 55 };
    let handle_y = if *is_pressed { 52 } else { 15 };

    html! {
        <div class="lamp-test-container">
            <svg
                viewBox="0 0 100 100"
                class="lamp-test-svg"
                onmousedown={onmousedown}
                onmouseup={onmouseup}
                onmouseleave={onmouseleave}
                ontouchstart={ontouchstart}
                ontouchend={ontouchend}
            >
                // White/gray background square
                <rect
                    x="5"
                    y="5"
                    width="90"
                    height="90"
                    rx="6"
                    fill="#e0e0e0"
                    stroke="#a0a0a0"
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

                // LAMP text (center of slider, top line)
                <text
                    x="50"
                    y={(slider_y + 16).to_string()}
                    font-size="12"
                    font-weight="bold"
                    fill="#2d3748"
                    font-family="Arial, sans-serif"
                    text-anchor="middle"
                >
                    {"LAMP"}
                </text>

                // TEST text (center of slider, bottom line)
                <text
                    x="50"
                    y={(slider_y + 30).to_string()}
                    font-size="12"
                    font-weight="bold"
                    fill="#2d3748"
                    font-family="Arial, sans-serif"
                    text-anchor="middle"
                >
                    {"TEST"}
                </text>
            </svg>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lamp_test_button_props() {
        let props = LampTestButtonProps {
            on_press: Callback::noop(),
            on_release: Callback::noop(),
            disabled: false,
        };
        assert!(!props.disabled);
    }

    #[test]
    fn test_lamp_test_button_disabled() {
        let props = LampTestButtonProps {
            on_press: Callback::noop(),
            on_release: Callback::noop(),
            disabled: true,
        };
        assert!(props.disabled);
    }
}
