// Circular Knob Component
//
// A 7-position rotary knob for IBM 1130 speed control.
// Positions: SS, SMC, INT RUN, RUN, SI, DISP, LOAD
// Ported from knob-lamps CircularKnob React component.

use yew::prelude::*;
use std::f64::consts::PI;

/// Speed mode positions for the IBM 1130
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SpeedMode {
    /// Single Step - clock advances one step
    SS = 0,
    /// Single Memory Cycle
    SMC = 1,
    /// Interrupt Run - Level 5 after each instruction
    IntRun = 2,
    /// Program Run - normal execution
    #[default]
    Run = 3,
    /// Single Instruction
    SI = 4,
    /// Display Core Storage
    Disp = 5,
    /// Load Core Storage
    Load = 6,
}

impl SpeedMode {
    /// Get the angle in degrees for this position
    pub fn angle(&self) -> f64 {
        match self {
            SpeedMode::SS => -120.0,
            SpeedMode::SMC => -80.0,
            SpeedMode::IntRun => -40.0,
            SpeedMode::Run => 0.0,
            SpeedMode::SI => 40.0,
            SpeedMode::Disp => 80.0,
            SpeedMode::Load => 120.0,
        }
    }

    /// Get the display label for this position
    pub fn label(&self) -> &'static str {
        match self {
            SpeedMode::SS => "SS",
            SpeedMode::SMC => "SMC",
            SpeedMode::IntRun => "INT RUN",
            SpeedMode::Run => "RUN",
            SpeedMode::SI => "SI",
            SpeedMode::Disp => "DISP",
            SpeedMode::Load => "LOAD",
        }
    }

    /// Get all positions in order
    pub fn all() -> [SpeedMode; 7] {
        [
            SpeedMode::SS,
            SpeedMode::SMC,
            SpeedMode::IntRun,
            SpeedMode::Run,
            SpeedMode::SI,
            SpeedMode::Disp,
            SpeedMode::Load,
        ]
    }

    /// Create from index (0-6)
    pub fn from_index(index: usize) -> Option<SpeedMode> {
        match index {
            0 => Some(SpeedMode::SS),
            1 => Some(SpeedMode::SMC),
            2 => Some(SpeedMode::IntRun),
            3 => Some(SpeedMode::Run),
            4 => Some(SpeedMode::SI),
            5 => Some(SpeedMode::Disp),
            6 => Some(SpeedMode::Load),
            _ => None,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct CircularKnobProps {
    /// Current position of the knob
    pub position: SpeedMode,
    /// Callback when position changes
    pub on_change: Callback<SpeedMode>,
    /// Whether the knob is disabled
    #[prop_or(false)]
    pub disabled: bool,
}

/// Calculate position on a circle
fn get_label_position(angle: f64, radius: f64) -> (f64, f64) {
    let rad = angle * PI / 180.0;
    let x = 150.0 + radius * rad.sin();
    let y = 150.0 - radius * rad.cos();
    (x, y)
}

#[function_component(CircularKnob)]
pub fn circular_knob(props: &CircularKnobProps) -> Html {
    let hovered_position = use_state(|| None::<SpeedMode>);

    // Determine display angle (hovered or current)
    let display_angle = match *hovered_position {
        Some(hovered) if hovered != props.position => hovered.angle(),
        _ => props.position.angle(),
    };

    let pointer_class = if hovered_position.is_some() && *hovered_position != Some(props.position) {
        "knob-pointer preview"
    } else {
        "knob-pointer"
    };

    html! {
        <div class="circular-knob-container">
            <svg width="300" height="300" viewBox="0 0 300 300">
                // Outer black disc
                <circle cx="150" cy="150" r="140" fill="#2a2a2a" />

                // Large hover/click areas around each label
                { for SpeedMode::all().iter().map(|&mode| {
                    if mode == props.position || props.disabled {
                        return html! {};
                    }

                    let (x, y) = get_label_position(mode.angle(), 105.0);
                    let hovered = hovered_position.clone();
                    let on_change = props.on_change.clone();

                    let onmouseenter = {
                        let hovered = hovered.clone();
                        Callback::from(move |_: MouseEvent| {
                            hovered.set(Some(mode));
                        })
                    };

                    let onmouseleave = {
                        let hovered = hovered.clone();
                        Callback::from(move |_: MouseEvent| {
                            hovered.set(None);
                        })
                    };

                    let onclick = Callback::from(move |_: MouseEvent| {
                        on_change.emit(mode);
                    });

                    html! {
                        <circle
                            class="clickable-area"
                            cx={x.to_string()}
                            cy={y.to_string()}
                            r="35"
                            fill="transparent"
                            style="cursor: pointer; pointer-events: all;"
                            onmouseenter={onmouseenter}
                            onmouseleave={onmouseleave}
                            onclick={onclick}
                        />
                    }
                })}

                // Position labels
                { for SpeedMode::all().iter().map(|&mode| {
                    let (x, y) = get_label_position(mode.angle(), 105.0);
                    let is_active = mode == props.position;
                    let is_hovered = *hovered_position == Some(mode);

                    let mut class = String::from("knob-label");
                    if is_active {
                        class.push_str(" active");
                    }
                    if is_hovered {
                        class.push_str(" hovered");
                    }

                    // Handle two-line label for "INT RUN"
                    if mode == SpeedMode::IntRun {
                        html! {
                            <text
                                x={x.to_string()}
                                y={y.to_string()}
                                text-anchor="middle"
                                dominant-baseline="middle"
                                class={class}
                                style="font-size: 22px; pointer-events: none;"
                            >
                                <tspan x={x.to_string()} dy="-0.4em">{"INT"}</tspan>
                                <tspan x={x.to_string()} dy="1em">{"RUN"}</tspan>
                            </text>
                        }
                    } else {
                        html! {
                            <text
                                x={x.to_string()}
                                y={y.to_string()}
                                text-anchor="middle"
                                dominant-baseline="middle"
                                class={class}
                                style="font-size: 22px; pointer-events: none;"
                            >
                                {mode.label()}
                            </text>
                        }
                    }
                })}

                // Center ring layers
                <circle cx="150" cy="150" r="45" fill="#4a4a4a" />
                <circle cx="150" cy="150" r="40" fill="#6a6a6a" />
                <circle cx="150" cy="150" r="35" fill="#5a5a5a" />

                // Pointer - rotates to current or hovered position
                <g
                    class={pointer_class}
                    style={format!("transform: rotate({}deg); transform-origin: 150px 150px;", display_angle)}
                >
                    // Pointer line
                    <line
                        x1="150"
                        y1="150"
                        x2="150"
                        y2="115"
                        stroke="#d0d0d0"
                        stroke-width="3"
                        stroke-linecap="round"
                    />
                    // Pointer tip
                    <circle cx="150" cy="115" r="4" fill="#d0d0d0" />
                </g>

                // Center button
                <circle cx="150" cy="150" r="25" fill="#3a3a3a" />
                <circle cx="150" cy="150" r="20" fill="#2a2a2a" />
            </svg>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speed_mode_angles() {
        assert_eq!(SpeedMode::SS.angle(), -120.0);
        assert_eq!(SpeedMode::Run.angle(), 0.0);
        assert_eq!(SpeedMode::Load.angle(), 120.0);
    }

    #[test]
    fn test_speed_mode_labels() {
        assert_eq!(SpeedMode::SS.label(), "SS");
        assert_eq!(SpeedMode::IntRun.label(), "INT RUN");
        assert_eq!(SpeedMode::Load.label(), "LOAD");
    }

    #[test]
    fn test_speed_mode_from_index() {
        assert_eq!(SpeedMode::from_index(0), Some(SpeedMode::SS));
        assert_eq!(SpeedMode::from_index(3), Some(SpeedMode::Run));
        assert_eq!(SpeedMode::from_index(6), Some(SpeedMode::Load));
        assert_eq!(SpeedMode::from_index(7), None);
    }

    #[test]
    fn test_speed_mode_all() {
        let all = SpeedMode::all();
        assert_eq!(all.len(), 7);
        assert_eq!(all[0], SpeedMode::SS);
        assert_eq!(all[6], SpeedMode::Load);
    }

    #[test]
    fn test_label_position() {
        let (x, y) = get_label_position(0.0, 100.0);
        // At 0 degrees, should be at top (150, 50)
        assert!((x - 150.0).abs() < 0.01);
        assert!((y - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_default_mode() {
        assert_eq!(SpeedMode::default(), SpeedMode::Run);
    }
}
