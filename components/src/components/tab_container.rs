// Tab Container Component
//
// Provides a tabbed interface for the IBM 1130 system emulator.
// Tabs: Keypunch | Printer | Assembler Game | Console

use yew::prelude::*;

/// Available tabs in the system
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Tab {
    Keypunch,
    Printer,
    #[default]
    Assembler,
    Console,
}

impl Tab {
    /// Get the display label for this tab
    pub fn label(&self) -> &'static str {
        match self {
            Tab::Keypunch => "Keypunch",
            Tab::Printer => "Printer",
            Tab::Assembler => "Assembler Game",
            Tab::Console => "Console",
        }
    }

    /// Get all tabs in order
    pub fn all() -> [Tab; 4] {
        [Tab::Keypunch, Tab::Printer, Tab::Assembler, Tab::Console]
    }
}

/// Tab Navigation Component - can be placed inside header
#[derive(Properties, PartialEq)]
pub struct TabNavProps {
    pub active_tab: Tab,
    #[prop_or_default]
    pub on_tab_change: Callback<Tab>,
    /// Show help button (only when on Console tab)
    #[prop_or(false)]
    pub show_help_button: bool,
    /// Help button active state
    #[prop_or(false)]
    pub help_active: bool,
    /// Callback when help button is clicked
    #[prop_or_default]
    pub on_help_toggle: Callback<()>,
}

#[function_component(TabNav)]
pub fn tab_nav(props: &TabNavProps) -> Html {
    let on_tab_click = |tab: Tab| {
        let on_tab_change = props.on_tab_change.clone();
        Callback::from(move |_: MouseEvent| {
            on_tab_change.emit(tab);
        })
    };

    let on_help_click = {
        let on_help_toggle = props.on_help_toggle.clone();
        Callback::from(move |_: MouseEvent| {
            on_help_toggle.emit(());
        })
    };

    html! {
        <nav class="tab-nav" role="tablist">
            { for Tab::all().iter().map(|&tab| {
                let is_active = props.active_tab == tab;
                let class = if is_active { "tab-button active" } else { "tab-button" };

                html! {
                    <button
                        class={class}
                        role="tab"
                        aria-selected={is_active.to_string()}
                        onclick={on_tab_click(tab)}
                    >
                        {tab.label()}
                    </button>
                }
            })}
            if props.show_help_button {
                <button
                    class={classes!("tab-help-btn", props.help_active.then_some("active"))}
                    onclick={on_help_click}
                    title="Show help for Console panel"
                >
                    {"?"}
                </button>
            }
        </nav>
    }
}

#[derive(Properties, PartialEq)]
pub struct TabContainerProps {
    /// Current active tab
    #[prop_or_default]
    pub active_tab: Tab,
    /// Callback when tab changes
    #[prop_or_default]
    pub on_tab_change: Callback<Tab>,
    /// Children to render in each tab
    #[prop_or_default]
    pub children: Children,
    /// Content for Keypunch tab
    #[prop_or_default]
    pub keypunch_content: Html,
    /// Content for Printer tab
    #[prop_or_default]
    pub printer_content: Html,
    /// Content for Assembler tab
    #[prop_or_default]
    pub assembler_content: Html,
    /// Content for Console tab
    #[prop_or_default]
    pub console_content: Html,
}

#[function_component(TabContainer)]
pub fn tab_container(props: &TabContainerProps) -> Html {
    // Tab content only - navigation is handled by TabNav in header
    html! {
        <div class="tab-container">
            <div class="tab-content" role="tabpanel">
                { match props.active_tab {
                    Tab::Keypunch => props.keypunch_content.clone(),
                    Tab::Printer => props.printer_content.clone(),
                    Tab::Assembler => props.assembler_content.clone(),
                    Tab::Console => props.console_content.clone(),
                }}
            </div>
        </div>
    }
}

/// Placeholder component for unimplemented tabs
#[derive(Properties, PartialEq)]
pub struct PlaceholderProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub description: Option<AttrValue>,
}

#[function_component(TabPlaceholder)]
pub fn tab_placeholder(props: &PlaceholderProps) -> Html {
    html! {
        <div class="tab-placeholder">
            <h2 class="placeholder-title">{&props.title}</h2>
            if let Some(desc) = &props.description {
                <p class="placeholder-description">{desc}</p>
            }
            <div class="placeholder-icon">
                {"Coming Soon"}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_labels() {
        assert_eq!(Tab::Keypunch.label(), "Keypunch");
        assert_eq!(Tab::Printer.label(), "Printer");
        assert_eq!(Tab::Assembler.label(), "Assembler Game");
        assert_eq!(Tab::Console.label(), "Console");
    }

    #[test]
    fn test_tab_all() {
        let all = Tab::all();
        assert_eq!(all.len(), 4);
        assert_eq!(all[0], Tab::Keypunch);
        assert_eq!(all[3], Tab::Console);
    }

    #[test]
    fn test_default_tab() {
        assert_eq!(Tab::default(), Tab::Assembler);
    }
}
