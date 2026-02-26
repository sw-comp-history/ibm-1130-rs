mod header;
mod memory_viewer;
mod modal;
mod program_area;
mod register_panel;
mod sidebar;

// Console panel components
pub mod circular_knob;
pub mod console_panel;
pub mod emergency_stop;
pub mod indicator_lights;
pub mod keypunch;
pub mod lamp_test_button;
pub mod power_switch;
pub mod printer;
pub mod sixteen_bit_panel;
pub mod tab_container;
pub mod toggle_switch;

pub use header::*;
pub use memory_viewer::*;
pub use modal::*;
pub use program_area::*;
pub use register_panel::*;
pub use sidebar::*;

// Re-export console panel components
pub use circular_knob::{CircularKnob, SpeedMode};
pub use console_panel::{ConsolePanel, ConsoleState, ConsoleAction, Registers};
pub use emergency_stop::EmergencyStop;
pub use indicator_lights::{IndicatorLights, RegisterDisplay};
pub use keypunch::{Keypunch, Deck, PunchCardSvg};
pub use lamp_test_button::LampTestButton;
pub use power_switch::PowerSwitch;
pub use sixteen_bit_panel::{SixteenBitPanel, PanelMode};
pub use tab_container::{Tab, TabContainer, TabNav, TabPlaceholder};
pub use toggle_switch::ToggleSwitch;
pub use printer::{Printer, PrinterState, sample_assembler_listing};
