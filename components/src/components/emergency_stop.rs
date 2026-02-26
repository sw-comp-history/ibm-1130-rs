// Emergency Stop Pull Switch Component
//
// Static painted metal pull knob with white lettering
// Non-interactive decorative element (used for fires only)
// Displayed in a white square panel

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EmergencyStopProps {}

#[function_component(EmergencyStop)]
pub fn emergency_stop(_props: &EmergencyStopProps) -> Html {
    html! {
        <div class="emergency-stop-panel">
            <div class="emergency-stop-knob">
                <span class="emergency-text">{"EMERGENCY"}</span>
                <span class="stop-text">{"STOP"}</span>
            </div>
        </div>
    }
}
