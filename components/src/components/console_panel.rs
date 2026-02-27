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
    Deposit,
    DepositNext,
    Examine,
    ExamineNext,
    Reset,
    ToggleRunning,
    UpdateRegisters(Registers),
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
            ConsoleAction::Deposit => {
                if new_state.power_on {
                    new_state.registers.sbr = new_state.switches;
                }
            }
            ConsoleAction::DepositNext => {
                if new_state.power_on {
                    new_state.registers.iar = new_state.registers.iar.wrapping_add(1);
                    new_state.registers.sbr = new_state.switches;
                }
            }
            ConsoleAction::Examine => {
                if new_state.power_on {
                    new_state.registers.sar = new_state.registers.iar;
                }
            }
            ConsoleAction::ExamineNext => {
                if new_state.power_on {
                    new_state.registers.iar = new_state.registers.iar.wrapping_add(1);
                    new_state.registers.sar = new_state.registers.iar;
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
            ConsoleAction::UpdateRegisters(regs) => {
                new_state.registers = regs;
            }
        }

        std::rc::Rc::new(new_state)
    }
}

#[derive(Properties, PartialEq)]
pub struct ConsolePanelProps {
    #[prop_or_default]
    pub on_state_change: Callback<ConsoleState>,
    #[prop_or_default]
    pub external_registers: Option<Registers>,
    #[prop_or_default]
    pub on_load: Callback<u16>,
    #[prop_or_default]
    pub on_deposit: Callback<(u16, u16)>,
    #[prop_or_default]
    pub on_examine: Callback<u16>,
    #[prop_or_default]
    pub on_start_stop: Callback<bool>,
    #[prop_or_default]
    pub on_reset: Callback<()>,
}

#[function_component(ConsolePanel)]
pub fn console_panel(props: &ConsolePanelProps) -> Html {
    let state = use_reducer(ConsoleState::default);

    {
        let state = state.clone();
        let external_registers = props.external_registers;
        use_effect_with(external_registers, move |regs| {
            if let Some(regs) = regs {
                state.dispatch(ConsoleAction::UpdateRegisters(*regs));
            }
            || ()
        });
    }

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
        Callback::from(move |_| {
            state.dispatch(ConsoleAction::TogglePower);
        })
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

    let _on_deposit = {
        let state = state.clone();
        let callback = props.on_deposit.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(ConsoleAction::Deposit);
            callback.emit((state.registers.iar, state.switches));
        })
    };

    let _on_deposit_next = {
        let state = state.clone();
        let callback = props.on_deposit.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(ConsoleAction::DepositNext);
            callback.emit((state.registers.iar.wrapping_add(1), state.switches));
        })
    };

    let on_examine = {
        let state = state.clone();
        let callback = props.on_examine.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(ConsoleAction::Examine);
            callback.emit(state.registers.iar);
        })
    };

    let _on_examine_next = {
        let state = state.clone();
        let callback = props.on_examine.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(ConsoleAction::ExamineNext);
            callback.emit(state.registers.iar.wrapping_add(1));
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

    let on_start = {
        let state = state.clone();
        let callback = props.on_start_stop.clone();
        Callback::from(move |_: MouseEvent| {
            if !state.running {
                state.dispatch(ConsoleAction::ToggleRunning);
                callback.emit(true);
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
                    // Left/Center section: Register indicators (6 rows × 16)
                    <RegisterDisplay
                        iar={state.registers.iar}
                        sar={state.registers.sar}
                        sbr={state.registers.sbr}
                        afr={state.registers.afr}
                        acc={state.registers.acc}
                        ext={state.registers.ext}
                        lamp_test={state.lamp_test}
                        power_on={state.power_on}
                    />
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
                <div class="knob-panel">
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
                    show_value_display={true}
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
                        <div class={classes!("status-light", "orange-light", (state.lamp_test || state.power_on).then_some("lit"))}>
                            <div class="light-line" />
                            <span>{"DISK"}</span>
                            <span>{"UNLOCK"}</span>
                            <div class="light-line" />
                        </div>
                        <div class={classes!("status-light", "darkgreen-light", (state.lamp_test).then_some("lit"))}>
                            <div class="light-line" />
                            <span>{"FILE"}</span>
                            <span>{"READY"}</span>
                            <div class="light-line" />
                        </div>
                    </div>
                    // Row 3: Green RUN, Red PARITY CHECK
                    <div class="status-light-row">
                        <div class={classes!("status-light", "green-light", (state.lamp_test || state.running).then_some("lit"))}>
                            <div class="light-line" />
                            <span>{"RUN"}</span>
                            <div class="light-line" />
                        </div>
                        <div class={classes!("status-light", "red-light", (state.lamp_test).then_some("lit"))}>
                            <div class="light-line" />
                            <span>{"PARITY"}</span>
                            <span>{"CHECK"}</span>
                            <div class="light-line" />
                        </div>
                    </div>
                    // Row 4: Gray K.B. SELECT, Yellow FORMS CHECK
                    <div class="status-light-row">
                        <div class={classes!("status-light", "gray-light", (state.lamp_test).then_some("lit"))}>
                            <div class="light-line" />
                            <span>{"K.B."}</span>
                            <span>{"SELECT"}</span>
                            <div class="light-line" />
                        </div>
                        <div class={classes!("status-light", "yellow-light", (state.lamp_test).then_some("lit"))}>
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
                    <PowerSwitch is_on={state.power_on} on_toggle={on_power_toggle} />
                    <div class="console-keyboard-switch">
                        <svg viewBox="0 0 100 100" class="kb-switch-svg">
                            // White/gray background square
                            <rect x="5" y="5" width="90" height="90" rx="6" fill="#e0e0e0" stroke="#a0a0a0" stroke-width="2"/>
                            // Dark vertical handle
                            <rect x="38" y="15" width="24" height="38" rx="3" fill="#1a1a1a"/>
                            // White sliding toggle bar (up position = CONSOLE)
                            <rect x="8" y="55" width="84" height="38" rx="4" fill="#f8f8f8" stroke="#c0c0c0" stroke-width="1"/>
                            // CON text
                            <text x="30" y="78" font-size="10" font-weight="bold" fill="#2d3748" font-family="Arial, sans-serif">{"CON"}</text>
                            // INT text
                            <text x="58" y="78" font-size="10" font-weight="bold" fill="#2d3748" font-family="Arial, sans-serif">{"INT"}</text>
                        </svg>
                    </div>
                    // Row 2: PROGRAM START, IMM STOP
                    <button class="console-btn green" onclick={on_start.clone()} disabled={button_disabled || state.running}>{"PROG START"}</button>
                    <button class="console-btn red" onclick={on_stop.clone()} disabled={button_disabled}>{"IMM STOP"}</button>
                    // Row 3: PROGRAM STOP, RESET
                    <button class="console-btn gray" onclick={on_stop.clone()} disabled={button_disabled || !state.running}>{"PROG STOP"}</button>
                    <button class="console-btn blue" onclick={on_reset.clone()} disabled={button_disabled}>{"RESET"}</button>
                    // Row 4: LOAD IAR, PROGRAM LOAD
                    <button class="console-btn blue" onclick={on_load.clone()} disabled={button_disabled}>{"LOAD IAR"}</button>
                    <button class="console-btn blue" onclick={on_examine.clone()} disabled={button_disabled}>{"PROG LOAD"}</button>
                </div>
            </div>

            // Bottom Right: Lamp Test (CE Panel - hidden on real hardware)
            <div class="lamp-test-section">
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
