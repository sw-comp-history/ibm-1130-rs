// Console Panel Component
//
// IBM 1130 Console Panel layout matching the physical console:
// - Top section: Indicator lights (left 3/4) + Speed knob (right 1/4)
// - Middle section: Console Entry Switches (printer front panel style)
// - Bottom section: Control buttons arranged around keyboard

use yew::prelude::*;
use crate::components::circular_knob::{CircularKnob, SpeedMode};
use crate::components::emergency_stop::EmergencyStop;
use crate::components::indicator_lights::{RegisterDisplay, ControlDisplay};
use crate::components::power_switch::PowerSwitch;
use crate::components::sixteen_bit_panel::SixteenBitPanel;

/// IBM 1130 Register State
#[derive(Clone, Copy, PartialEq, Default)]
pub struct Registers {
    pub acc: u16,
    pub ext: u16,
    pub iar: u16,
    pub sar: u16,
    pub sbr: u16,
    pub afr: u16,
}

/// IBM 1130 Control/Status State (right-side indicators)
#[derive(Clone, Copy, PartialEq, Default)]
pub struct ControlState {
    /// Operation code (5 bits)
    pub op_code: u8,
    /// Format bit (long instruction)
    pub format: bool,
    /// Tag bits (index register selection, 2 bits)
    pub tag: u8,
    /// Cycle/timing indicator (0-7)
    pub cycle: u8,
    /// Wait state
    pub wait: bool,
    /// Indirect addressing
    pub indirect: bool,
    /// Carry flag
    pub carry: bool,
    /// Overflow flag
    pub overflow: bool,
}

/// Console Panel State
#[derive(Clone, PartialEq)]
pub struct ConsoleState {
    pub switches: u16,
    pub registers: Registers,
    pub control: ControlState,
    pub speed_mode: SpeedMode,
    pub power_on: bool,
    pub lamp_test: bool,
    pub running: bool,
    pub help_active: bool,
    /// CONSOLE / KEYBOARD switch: true when up (CON, programs read the
    /// console entry switches), false when down (INT, the keyboard)
    pub console_entry: bool,
}

impl Default for ConsoleState {
    fn default() -> Self {
        Self {
            switches: 0,
            registers: Registers::default(),
            control: ControlState::default(),
            speed_mode: SpeedMode::Run,
            power_on: false,
            lamp_test: false,
            running: false,
            help_active: false,
            console_entry: false,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum ConsoleAction {
    SetSwitches(u16),
    SetSpeedMode(SpeedMode),
    TogglePower,
    SetLampTest(bool),
    Load,
    Store,
    Display,
    Reset,
    ToggleRunning,
    ToggleHelp,
    SetHelpActive(bool),
    ToggleConsoleEntry,
}

impl Reducible for ConsoleState {
    type Action = ConsoleAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut new_state = (*self).clone();

        match action {
            ConsoleAction::SetSwitches(value) => {
                new_state.switches = value;
            }
            ConsoleAction::SetSpeedMode(mode) => {
                new_state.speed_mode = mode;
            }
            ConsoleAction::TogglePower => {
                new_state.power_on = !new_state.power_on;
                if !new_state.power_on {
                    new_state.running = false;
                }
            }
            ConsoleAction::SetLampTest(active) => {
                new_state.lamp_test = active;
            }
            ConsoleAction::Load => {
                if new_state.power_on {
                    new_state.registers.iar = new_state.switches;
                }
            }
            // LOAD mode PROG START: store the entry switches at IAR, advance IAR
            ConsoleAction::Store => {
                if new_state.power_on {
                    new_state.registers.sar = new_state.registers.iar;
                    new_state.registers.sbr = new_state.switches;
                    new_state.registers.iar = new_state.registers.iar.wrapping_add(1);
                }
            }
            // DISP mode PROG START: address the word at IAR, advance IAR
            // (the word itself comes from the connected emulator's memory)
            ConsoleAction::Display => {
                if new_state.power_on {
                    new_state.registers.sar = new_state.registers.iar;
                    new_state.registers.iar = new_state.registers.iar.wrapping_add(1);
                }
            }
            ConsoleAction::Reset => {
                new_state.registers = Registers::default();
                new_state.running = false;
            }
            ConsoleAction::ToggleRunning => {
                if new_state.power_on {
                    new_state.running = !new_state.running;
                }
            }
            ConsoleAction::ToggleHelp => {
                new_state.help_active = !new_state.help_active;
            }
            ConsoleAction::SetHelpActive(active) => {
                new_state.help_active = active;
            }
            ConsoleAction::ToggleConsoleEntry => {
                new_state.console_entry = !new_state.console_entry;
            }
        }

        std::rc::Rc::new(new_state)
    }
}

#[derive(Properties, PartialEq)]
pub struct ConsolePanelProps {
    #[prop_or_default]
    pub on_state_change: Callback<ConsoleState>,
    /// Registers from a connected emulator; when set they are displayed
    /// instead of the panel's own registers
    #[prop_or_default]
    pub external_registers: Option<Registers>,
    /// LOAD IAR pressed: entry switch value to load into IAR
    #[prop_or_default]
    pub on_load: Callback<u16>,
    /// PROG START in LOAD mode: (address, word) to store in memory
    #[prop_or_default]
    pub on_deposit: Callback<(u16, u16)>,
    /// PROG START in DISP mode: address of the word to display
    #[prop_or_default]
    pub on_examine: Callback<u16>,
    #[prop_or_default]
    pub on_start_stop: Callback<bool>,
    #[prop_or_default]
    pub on_reset: Callback<()>,
    /// External help state (controlled from header)
    #[prop_or(false)]
    pub help_active: bool,
    /// Callback when power switch is clicked (to dismiss help)
    #[prop_or_default]
    pub on_power_click: Callback<()>,
}

#[function_component(ConsolePanel)]
pub fn console_panel(props: &ConsolePanelProps) -> Html {
    let state = use_reducer(ConsoleState::default);
    let registers = props.external_registers.unwrap_or(state.registers);

    let on_switch_change = {
        let state = state.clone();
        Callback::from(move |value: u16| {
            state.dispatch(ConsoleAction::SetSwitches(value));
        })
    };

    let on_speed_change = {
        let state = state.clone();
        Callback::from(move |mode: SpeedMode| {
            state.dispatch(ConsoleAction::SetSpeedMode(mode));
        })
    };

    let on_power_toggle = {
        let state = state.clone();
        let on_power_click = props.on_power_click.clone();
        Callback::from(move |_| {
            state.dispatch(ConsoleAction::TogglePower);
            on_power_click.emit(());
        })
    };

    let on_console_entry_toggle = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(ConsoleAction::ToggleConsoleEntry);
        })
    };

    // CONSOLE / KEYBOARD switch: the white toggle bar sits in the half
    // the switch is set to, with the dark handle in the other half
    let (kb_handle_y, kb_bar_y) = if state.console_entry { (47, 7) } else { (15, 55) };
    let (con_fill, int_fill) = if state.console_entry {
        ("#2d3748", "#a0aec0")
    } else {
        ("#a0aec0", "#2d3748")
    };

    let on_lamp_test_press = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(ConsoleAction::SetLampTest(true));
        })
    };

    let on_lamp_test_release = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(ConsoleAction::SetLampTest(false));
        })
    };

    let on_load = {
        let state = state.clone();
        let callback = props.on_load.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(ConsoleAction::Load);
            callback.emit(state.switches);
        })
    };

    let on_reset = {
        let state = state.clone();
        let callback = props.on_reset.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(ConsoleAction::Reset);
            callback.emit(());
        })
    };

    // PROG START behavior depends on the mode switch
    let on_start = {
        let state = state.clone();
        let on_start_stop = props.on_start_stop.clone();
        let on_deposit = props.on_deposit.clone();
        let on_examine = props.on_examine.clone();
        Callback::from(move |_: MouseEvent| match state.speed_mode {
            SpeedMode::Load => {
                on_deposit.emit((registers.iar, state.switches));
                state.dispatch(ConsoleAction::Store);
            }
            SpeedMode::Disp => {
                on_examine.emit(registers.iar);
                state.dispatch(ConsoleAction::Display);
            }
            _ => {
                if !state.running {
                    state.dispatch(ConsoleAction::ToggleRunning);
                    on_start_stop.emit(true);
                }
            }
        })
    };

    let on_stop = {
        let state = state.clone();
        let callback = props.on_start_stop.clone();
        Callback::from(move |_: MouseEvent| {
            if state.running {
                state.dispatch(ConsoleAction::ToggleRunning);
                callback.emit(false);
            }
        })
    };

    let button_disabled = !state.power_on;

    html! {
        <div class="console-panel">
            // Top Section: Emergency Stop | Indicator Lights (3 groups) | Knob
            <div class="console-top">
                <EmergencyStop />
                <div class="lights-panel">
                    // Left section: Register indicators (6 rows × 16)
                    <RegisterDisplay
                        iar={registers.iar}
                        sar={registers.sar}
                        sbr={registers.sbr}
                        afr={registers.afr}
                        acc={registers.acc}
                        ext={registers.ext}
                        lamp_test={state.lamp_test}
                        power_on={state.power_on}
                    />
                    // Middle section: Black unlabeled backlit panel
                    <div class="center-panel">
                        <div class="center-panel-inner" />
                    </div>
                    // Right section: Control/Status indicators
                    <ControlDisplay
                        op_code={state.control.op_code}
                        format={state.control.format}
                        tag={state.control.tag}
                        cycle={state.control.cycle}
                        wait={state.control.wait}
                        run={state.running}
                        indirect={state.control.indirect}
                        carry={state.control.carry}
                        overflow={state.control.overflow}
                        lamp_test={state.lamp_test}
                        power_on={state.power_on}
                    />
                </div>
                <div class="knob-panel tip-below tip-slow" data-tip={TIP_MODE_SWITCH}>
                    <CircularKnob
                        position={state.speed_mode}
                        on_change={on_speed_change}
                        disabled={!state.power_on}
                    />
                </div>
            </div>

            // Middle Section: Toggle Switches
            <div class="console-switches">
                <SixteenBitPanel
                    value={state.switches}
                    on_change={on_switch_change}
                    label=""
                    show_value_display={false}
                />
            </div>

            // Bottom Section: Keyboard with status lights and switches
            <div class="console-keyboard-area">
                // Left side: Status indicator lights (matching photo layout)
                <div class="status-lights-panel">
                    // Row 1: Gray, Gray (unlabeled in photo)
                    <div class="status-light-row">
                        <div class={classes!("status-light", "gray-light", (state.lamp_test).then_some("lit"))} />
                        <div class={classes!("status-light", "gray-light", (state.lamp_test).then_some("lit"))} />
                    </div>
                    // Row 2: Orange DISK UNLOCK, Dark Green FILE READY
                    <div class="status-light-row">
                        <div class={classes!("status-light", "orange-light", (state.lamp_test || state.power_on).then_some("lit"))}
                            data-tip="DISK UNLOCK — the disk drive is stopped and its cartridge can be removed">
                            <div class="light-line" />
                            <span>{"DISK"}</span>
                            <span>{"UNLOCK"}</span>
                            <div class="light-line" />
                        </div>
                        <div class={classes!("status-light", "darkgreen-light", (state.lamp_test).then_some("lit"))}
                            data-tip="FILE READY — the disk drive is up to speed and ready for use">
                            <div class="light-line" />
                            <span>{"FILE"}</span>
                            <span>{"READY"}</span>
                            <div class="light-line" />
                        </div>
                    </div>
                    // Row 3: Green RUN, Red PARITY CHECK
                    <div class="status-light-row">
                        <div class={classes!("status-light", "green-light", (state.lamp_test || state.running).then_some("lit"))}
                            data-tip="RUN — the processor is executing instructions">
                            <div class="light-line" />
                            <span>{"RUN"}</span>
                            <div class="light-line" />
                        </div>
                        <div class={classes!("status-light", "red-light", (state.lamp_test).then_some("lit"))}
                            data-tip="PARITY CHECK — a core storage parity error was detected">
                            <div class="light-line" />
                            <span>{"PARITY"}</span>
                            <span>{"CHECK"}</span>
                            <div class="light-line" />
                        </div>
                    </div>
                    // Row 4: Gray K.B. SELECT, Yellow FORMS CHECK
                    <div class="status-light-row">
                        <div class={classes!("status-light", "gray-light", (state.lamp_test).then_some("lit"))}
                            data-tip="K.B. SELECT — the console keyboard is selected for input">
                            <div class="light-line" />
                            <span>{"K.B."}</span>
                            <span>{"SELECT"}</span>
                            <div class="light-line" />
                        </div>
                        <div class={classes!("status-light", "yellow-light", (state.lamp_test).then_some("lit"))}
                            data-tip="FORMS CHECK — the console printer is out of paper">
                            <div class="light-line" />
                            <span>{"FORMS"}</span>
                            <span>{"CHECK"}</span>
                            <div class="light-line" />
                        </div>
                    </div>
                </div>

                // Center: Keyboard SVG
                <div class="keyboard-center">
                    {render_keyboard_svg()}
                </div>

                // Right side: Switches and control buttons
                <div class="button-grid right-buttons">
                    // Row 1: Power switch and Console/Int Keyboard switch (white)
                    <div class={classes!("power-switch-wrapper", "tip-slow", props.help_active.then_some("help-blink"))}
                        data-tip={(!props.help_active).then_some("POWER — turn the system on or off")}>
                        <PowerSwitch is_on={state.power_on} on_toggle={on_power_toggle} />
                        if props.help_active {
                            <div class="help-tooltip">{"Click power to start."}</div>
                        }
                    </div>
                    <div class="console-keyboard-switch tip-slow"
                        data-tip={format!(
                            "CONSOLE / KEYBOARD — choose whether programs read input from the console entry switches (CON, up) or the keyboard (INT, down). Set to {}.",
                            if state.console_entry { "CON" } else { "INT" }
                        )}
                        onclick={on_console_entry_toggle}>
                        <svg viewBox="0 0 100 100" class="kb-switch-svg">
                            // White/gray background square
                            <rect x="5" y="5" width="90" height="90" rx="6" fill="#e0e0e0" stroke="#a0a0a0" stroke-width="2"/>
                            // Dark vertical handle
                            <rect x="38" y={kb_handle_y.to_string()} width="24" height="38" rx="3" fill="#1a1a1a"/>
                            // White sliding toggle bar (up position = CONSOLE)
                            <rect x="8" y={kb_bar_y.to_string()} width="84" height="38" rx="4" fill="#f8f8f8" stroke="#c0c0c0" stroke-width="1"/>
                            // CON text (dark when selected)
                            <text x="30" y={(kb_bar_y + 23).to_string()} font-size="10" font-weight="bold" fill={con_fill} font-family="Arial, sans-serif">{"CON"}</text>
                            // INT text (dark when selected)
                            <text x="58" y={(kb_bar_y + 23).to_string()} font-size="10" font-weight="bold" fill={int_fill} font-family="Arial, sans-serif">{"INT"}</text>
                        </svg>
                    </div>
                    // Row 2: PROGRAM START, IMM STOP
                    {console_button("PROG START", "green", TIP_PROG_START, Some(on_start), button_disabled || state.running)}
                    {console_button("IMM STOP", "red", "IMMEDIATE STOP — stop at once, even in the middle of an instruction", Some(on_stop.clone()), button_disabled)}
                    // Row 3: PROGRAM STOP, RESET
                    {console_button("PROG STOP", "gray", "PROGRAM STOP — stop at the end of the current instruction", Some(on_stop), button_disabled || !state.running)}
                    {console_button("RESET", "blue", "RESET — clear the registers and stop the processor", Some(on_reset), button_disabled)}
                    // Row 4: LOAD IAR, PROGRAM LOAD
                    {console_button("LOAD IAR", "blue", "LOAD IAR — copy the console entry switches into the Instruction Address Register", Some(on_load), button_disabled)}
                    {console_button("PROG LOAD", "blue", "PROGRAM LOAD — read a boot card from the card reader and start it (not yet emulated)", None, button_disabled)}
                </div>
            </div>

            // Bottom Right: Lamp Test (CE Panel - hidden on real hardware)
            <div class="lamp-test-section tip-slow tip-right-edge"
                data-tip="LAMP TEST — hold to light every lamp and check for burned-out bulbs">
                <span class="lamp-test-label">{"CE PANEL"}</span>
                <button
                    class="lamp-test-btn"
                    onmousedown={on_lamp_test_press}
                    onmouseup={on_lamp_test_release.clone()}
                    onmouseleave={on_lamp_test_release}
                    disabled={!state.power_on}
                >
                    {"LAMP TEST"}
                </button>
            </div>
        </div>
    }
}

const TIP_MODE_SWITCH: &str = "MODE SWITCH — sets what PROG START does: SS, SMC and SI step; \
INT RUN and RUN run; DISP shows and LOAD stores the word at IAR";

const TIP_PROG_START: &str = "PROGRAM START — RUN: start the program at IAR. \
LOAD: store the entry switches at IAR. DISP: show the word at IAR. \
LOAD and DISP then advance IAR to the next word";

/// Console push button. The tooltip goes on a wrapper because disabled
/// buttons are drawn at reduced opacity, which would also fade the tooltip.
fn console_button(
    label: &'static str,
    color: &'static str,
    tip: &'static str,
    onclick: Option<Callback<MouseEvent>>,
    disabled: bool,
) -> Html {
    html! {
        <span class="tip-holder tip-slow" data-tip={tip}>
            <button class={classes!("console-btn", color)} {onclick} {disabled}>{label}</button>
        </span>
    }
}

fn render_keyboard_svg() -> Html {
    html! {
        <svg class="keyboard-svg" viewBox="0 0 400 150" xmlns="http://www.w3.org/2000/svg">
            // Keyboard background
            <rect x="0" y="0" width="400" height="150" rx="6" fill="#1a1a1a"/>

            // Row 1: Number keys
            {(0..12).map(|i| {
                let x = 12 + i * 31;
                let keys = ["1","2","3","4","5","6","7","8","9","0","-","="];
                html! {
                    <g>
                        <rect x={x.to_string()} y="10" width="27" height="26" rx="3" fill="#555" stroke="#666" stroke-width="1"/>
                        <text x={(x+13).to_string()} y="28" text-anchor="middle" font-size="12" fill="#eee" font-weight="bold">{keys[i as usize]}</text>
                    </g>
                }
            }).collect::<Html>()}

            // Row 2: QWERTY
            {(0..12).map(|i| {
                let x = 22 + i * 31;
                let keys = ["Q","W","E","R","T","Y","U","I","O","P","[","]"];
                html! {
                    <g>
                        <rect x={x.to_string()} y="40" width="27" height="26" rx="3" fill="#888" stroke="#999" stroke-width="1"/>
                        <text x={(x+13).to_string()} y="58" text-anchor="middle" font-size="12" fill="#111" font-weight="bold">{keys[i as usize]}</text>
                    </g>
                }
            }).collect::<Html>()}

            // Row 3: Home row
            {(0..11).map(|i| {
                let x = 28 + i * 31;
                let keys = ["A","S","D","F","G","H","J","K","L",";","'"];
                html! {
                    <g>
                        <rect x={x.to_string()} y="70" width="27" height="26" rx="3" fill="#888" stroke="#999" stroke-width="1"/>
                        <text x={(x+13).to_string()} y="88" text-anchor="middle" font-size="12" fill="#111" font-weight="bold">{keys[i as usize]}</text>
                    </g>
                }
            }).collect::<Html>()}

            // Row 4: Bottom row
            {(0..10).map(|i| {
                let x = 44 + i * 31;
                let keys = ["Z","X","C","V","B","N","M",",",".","/"];
                html! {
                    <g>
                        <rect x={x.to_string()} y="100" width="27" height="26" rx="3" fill="#888" stroke="#999" stroke-width="1"/>
                        <text x={(x+13).to_string()} y="118" text-anchor="middle" font-size="12" fill="#111" font-weight="bold">{keys[i as usize]}</text>
                    </g>
                }
            }).collect::<Html>()}

            // Spacebar (blue like in image)
            <rect x="90" y="130" width="220" height="16" rx="3" fill="#3b82f6" stroke="#2563eb" stroke-width="1"/>
        </svg>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_console_state_default() {
        let state = ConsoleState::default();
        assert_eq!(state.switches, 0);
        assert!(!state.power_on);
        assert!(!state.running);
        assert_eq!(state.speed_mode, SpeedMode::Run);
    }

    #[test]
    fn test_registers_default() {
        let regs = Registers::default();
        assert_eq!(regs.acc, 0);
        assert_eq!(regs.ext, 0);
        assert_eq!(regs.iar, 0);
        assert_eq!(regs.sar, 0);
        assert_eq!(regs.sbr, 0);
        assert_eq!(regs.afr, 0);
    }

    #[test]
    fn test_store_advances_iar() {
        let state = ConsoleState {
            power_on: true,
            switches: 0xC0DE,
            registers: Registers {
                iar: 0x0100,
                ..Registers::default()
            },
            ..ConsoleState::default()
        };
        let state = std::rc::Rc::new(state).reduce(ConsoleAction::Store);
        assert_eq!(state.registers.sar, 0x0100);
        assert_eq!(state.registers.sbr, 0xC0DE);
        assert_eq!(state.registers.iar, 0x0101);

        let state = state.reduce(ConsoleAction::Display);
        assert_eq!(state.registers.sar, 0x0101);
        assert_eq!(state.registers.iar, 0x0102);
    }

    #[test]
    fn test_console_entry_switch_toggles() {
        let state = std::rc::Rc::new(ConsoleState::default());
        assert!(!state.console_entry);
        let state = state.reduce(ConsoleAction::ToggleConsoleEntry);
        assert!(state.console_entry);
        let state = state.reduce(ConsoleAction::ToggleConsoleEntry);
        assert!(!state.console_entry);
    }

    #[test]
    fn test_load_action() {
        let state = ConsoleState {
            power_on: true,
            switches: 0x1234,
            ..ConsoleState::default()
        };
        let state = std::rc::Rc::new(state);
        let new_state = state.reduce(ConsoleAction::Load);
        assert_eq!(new_state.registers.iar, 0x1234);
    }
}
