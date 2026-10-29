// Toggle Switch Component
//
// An interactive toggle switch that flips when clicked anywhere on it.
// Provides visual affordance through hover effects.
// Ported from toggle-nixie React component.

use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum HoverSide {
    Top,
    Bottom,
}

#[derive(Properties, PartialEq)]
pub struct ToggleSwitchProps {
    /// Current state of the switch
    pub is_on: bool,
    /// Callback when switch is toggled
    pub on_toggle: Callback<()>,
    /// The binary value this switch represents (8, 4, 2, or 1)
    #[prop_or(1)]
    pub value: u8,
    /// Whether the switch is disabled (non-interactive)
    #[prop_or(false)]
    pub disabled: bool,
}

#[function_component(ToggleSwitch)]
pub fn toggle_switch(props: &ToggleSwitchProps) -> Html {
    let hover_side = use_state(|| None::<HoverSide>);

    // A click anywhere on the switch flips it. Only accepting clicks on the
    // side opposite the knob left the knob itself unresponsive.
    let handle_click = {
        let on_toggle = props.on_toggle.clone();
        let disabled = props.disabled;
        Callback::from(move |_: MouseEvent| {
            if !disabled {
                on_toggle.emit(());
            }
        })
    };

    // While hovering, highlight the side the knob will move to
    let handle_mouse_enter = {
        let hover_side = hover_side.clone();
        let is_on = props.is_on;
        let disabled = props.disabled;
        Callback::from(move |_: MouseEvent| {
            if !disabled {
                hover_side.set(Some(if is_on {
                    HoverSide::Bottom
                } else {
                    HoverSide::Top
                }));
            }
        })
    };

    let handle_mouse_leave = {
        let hover_side = hover_side.clone();
        Callback::from(move |_: MouseEvent| {
            hover_side.set(None);
        })
    };

    let track_class = if props.is_on {
        "toggle-track on"
    } else {
        "toggle-track off"
    };

    let top_clickable_class = if *hover_side == Some(HoverSide::Top) {
        "toggle-clickable-area hover"
    } else {
        "toggle-clickable-area"
    };

    let bottom_clickable_class = if *hover_side == Some(HoverSide::Bottom) {
        "toggle-clickable-area hover"
    } else {
        "toggle-clickable-area"
    };

    let knob_class = if hover_side.is_some() {
        "toggle-knob preview"
    } else {
        "toggle-knob"
    };

    let highlight_class = if hover_side.is_some() {
        "toggle-knob-highlight preview"
    } else {
        "toggle-knob-highlight"
    };

    let knob_cy = if props.is_on { 35 } else { 85 };
    let highlight_cy = if props.is_on { 32 } else { 82 };

    let on_fill = if props.is_on { "#fff" } else { "#666" };
    let off_fill = if props.is_on { "#666" } else { "#fff" };

    let cursor = if props.disabled { "default" } else { "pointer" };

    html! {
        <div class="toggle-container">
            <div class="toggle-label">{props.value}</div>
            <svg
                width="60"
                height="120"
                viewBox="0 0 60 120"
                class="toggle-switch"
                aria-label={format!("Toggle switch for value {}, currently {}", props.value, if props.is_on { "on" } else { "off" })}
                role="switch"
                onclick={handle_click}
                onmouseenter={handle_mouse_enter}
                onmouseleave={handle_mouse_leave}
                style={format!("cursor: {}", cursor)}
            >
                // Switch background track (vertical)
                <rect
                    x="15"
                    y="10"
                    width="30"
                    height="100"
                    rx="15"
                    class={track_class}
                />

                // Top hover highlight (shown when switch is OFF)
                <rect
                    x="15"
                    y="10"
                    width="30"
                    height="50"
                    rx="15"
                    class={top_clickable_class}
                />

                // Bottom hover highlight (shown when switch is ON)
                <rect
                    x="15"
                    y="60"
                    width="30"
                    height="50"
                    rx="15"
                    class={bottom_clickable_class}
                />

                // ON label (top)
                <text
                    x="30"
                    y="33"
                    class="toggle-text"
                    fill={on_fill}
                    font-size="11"
                    font-weight="bold"
                    text-anchor="middle"
                >
                    {"1"}
                </text>

                // OFF label (bottom)
                <text
                    x="30"
                    y="93"
                    class="toggle-text"
                    fill={off_fill}
                    font-size="11"
                    font-weight="bold"
                    text-anchor="middle"
                >
                    {"0"}
                </text>

                // Toggle knob/handle (moves vertically)
                <circle
                    cx="30"
                    cy={knob_cy.to_string()}
                    r="18"
                    class={knob_class}
                />

                // Inner knob highlight for 3D effect
                <circle
                    cx="30"
                    cy={highlight_cy.to_string()}
                    r="10"
                    class={highlight_class}
                />
            </svg>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_switch_props() {
        let props = ToggleSwitchProps {
            is_on: true,
            on_toggle: Callback::noop(),
            value: 8,
            disabled: false,
        };
        assert!(props.is_on);
        assert_eq!(props.value, 8);
    }
}
