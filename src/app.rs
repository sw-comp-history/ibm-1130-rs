//! IBM 1130 Assembly Game - Yew Application
//!
//! Main application component using Yew framework and shared components
//! Integrated with tabbed interface for Keypunch, Printer, Assembler, and Console Panel

use components::{
    // Assembler game components
    Header, LegendItem, Modal, ProgramArea, Register, RegisterPanel, Sidebar, SidebarButton,
    WordMemoryViewer,
    // Tab container
    Tab, TabContainer, TabNav,
    // Console panel components
    ConsolePanel, Registers as ConsoleRegisters,
    // Keypunch component
    Keypunch, Deck,
    // Printer component
    Printer, sample_assembler_listing,
};
use yew::prelude::*;

use crate::challenge::{Challenge, get_all_challenges};
use crate::wasm::WasmCpu;

#[function_component(App)]
pub fn app() -> Html {
    // CPU state
    let cpu = use_state(WasmCpu::new);

    // Editor code
    let editor_code = use_state(|| {
        String::from(
            "       ORG  16        ; Start program at address 16 (0x10)\n; Example: Add two numbers\n       LD   0 30      ; Load value from address 30 into ACC\n       A    0 31      ; Add value from address 31 to ACC\n       STO  0 32      ; Store result at address 32\n       WAIT           ; Halt\n\n       ORG  30        ; Data section starting at address 30\n       DATA 30 5      ; Address 30: value 5\n       DATA 31 7      ; Address 31: value 7\n       DATA 32 0      ; Address 32: result (initially 0)",
        )
    });

    // Assembly output (Vec<String> for line-by-line display with highlighting)
    let assembly_lines = use_state(Vec::<String>::new);

    // Register change tracking
    let last_acc = use_state(|| 0u16);
    let last_ext = use_state(|| 0u16);
    let last_iar = use_state(|| 0u16);
    let last_xr1 = use_state(|| 0u16);
    let last_xr2 = use_state(|| 0u16);
    let last_xr3 = use_state(|| 0u16);

    // Memory change tracking (for memory-mapped registers)
    let changed_memory = use_state(Vec::<usize>::new);

    // Error message
    let error_message = use_state(|| None::<String>);

    // Modal states
    let tutorial_open = use_state(|| false);
    let examples_open = use_state(|| false);
    let challenges_open = use_state(|| false);
    let isa_open = use_state(|| false);
    let help_open = use_state(|| false);

    // Challenge state
    let current_challenge = use_state(|| None::<Challenge>);
    let challenge_result = use_state(|| None::<Result<String, String>>);

    // Tab state
    let active_tab = use_state(|| Tab::Assembler);

    // Keypunch deck state
    let keypunch_deck = use_state(Deck::default);

    // Printer state - lines to print
    let printer_content = use_state(sample_assembler_listing);

    // Console panel state is managed internally by ConsolePanel component

    // Load challenges using use_memo to avoid recreating on every render
    let challenges = use_memo((), |_| get_all_challenges());

    // Example programs
    let example_1 = "       ORG  16        ; Start program at address 16 (0x10)\n; Example 1: Add two numbers\n       LD   0 30      ; Load value from address 30 into ACC\n       A    0 31      ; Add value from address 31 to ACC\n       STO  0 32      ; Store result at address 32\n       WAIT           ; Halt\n\n       ORG  30        ; Data section starting at address 30\n       DATA 30 5      ; Address 30: value 5\n       DATA 31 7      ; Address 31: value 7\n       DATA 32 0      ; Address 32: result (initially 0)";
    let example_2 = "       ORG  16        ; Start program at address 16\n; Example 2: Indexed Addressing\n       LD   0 40      ; Load index value (3)\n       STX  1         ; Store in XR1\n       LD   1 30      ; Load from 30+XR1 = 33\n       WAIT\n\n       ORG  30        ; Data section\n       DATA 30 100\n       DATA 31 200\n       DATA 32 300\n       DATA 33 400    ; This will be loaded\n       DATA 40 3      ; Index value";
    let example_3 = "       ORG  16        ; Start program at address 16\n; Example 3: Shift Operations\n       LD   0 30      ; Load value (5)\n       SLA  2         ; Shift left 2 bits (5 * 4 = 20)\n       STO  0 31      ; Store result\n       WAIT\n\n       ORG  30        ; Data section\n       DATA 30 5\n       DATA 31 0";
    let example_4 = "       ORG  16        ; Start program at address 16\n; Example 4: Memory-Mapped Registers\n; IBM 1130 stores index registers XR1, XR2, XR3 in memory at word addresses 1, 2, 3\n; Watch these memory locations highlight when registers change!\n\n       LD   0 40      ; Load 100 into ACC\n       STO  0 1       ; Store to XR1 (word address 1)\n       LD   0 41      ; Load 200 into ACC\n       STO  0 2       ; Store to XR2 (word address 2)\n       LD   0 42      ; Load 300 into ACC\n       STO  0 3       ; Store to XR3 (word address 3)\n\n; Now use XR1 for indexed addressing\n       LD   1 50      ; Load from 50+XR1 = 150\n       WAIT\n\n       ORG  40        ; Data section\n       DATA 40 100    ; Value for XR1\n       DATA 41 200    ; Value for XR2\n       DATA 42 300    ; Value for XR3\n       DATA 150 999   ; Value at indexed address";
    let example_5 = "       ORG  16        ; Start program at address 16\n; Example 5: Special Memory Locations\n; IBM 1130 Architecture:\n;   Location 0: Safety trap (infinite loop if program crashes)\n;   Locations 1-3: Index registers XR1, XR2, XR3\n;   Locations 8-13: Interrupt vectors (priority levels 0-5)\n; By convention, location 0 contains an infinite loop instruction.\n; If a program jumps to 0 (error condition), IAR lights go dark.\n\n; This example demonstrates the special locations:\n       LD   0 50      ; Load a value\n       STO  0 1       ; Store to XR1 (location 1)\n       STO  0 2       ; Store to XR2 (location 2)\n       STO  0 3       ; Store to XR3 (location 3)\n       WAIT\n\n       ORG  50        ; Data section\n       DATA 50 42     ; Test value\n\n; Note: In a real 1130, location 0 would contain:\n; DATA 0 0x4800  ; Opcode for 'BSI 0' (branch to self)";

    // Callbacks for ProgramArea
    let on_assemble = {
        let cpu = cpu.clone();
        let assembly_lines = assembly_lines.clone();
        let error_message = error_message.clone();
        Callback::from(move |code: String| {
            let mut cpu_mut = (*cpu).clone();
            cpu_mut.hard_reset();

            // Parse DATA directives and load them into memory
            for line in code.lines() {
                let trimmed = line.trim();
                if trimmed.to_uppercase().starts_with("DATA") {
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    if parts.len() >= 3
                        && let (Ok(addr), Ok(value)) =
                            (parts[1].parse::<u16>(), parts[2].parse::<u16>())
                        {
                            let _ = cpu_mut.write_memory(addr, value);
                        }
                }
            }

            // Assemble the program (load at PROGRAM_START = 0x0010)
            match cpu_mut.assemble(code, crate::cpu::PROGRAM_START) {
                Ok(listing_js) => {
                    // Parse the listing
                    if let Ok(listing) =
                        serde_wasm_bindgen::from_value::<Vec<serde_json::Value>>(listing_js)
                    {
                        let lines: Vec<String> = listing
                            .iter()
                            .map(|line| {
                                let addr = line["address"].as_u64().unwrap_or(0) as u16;
                                let opcode = line["opcode"].as_str().unwrap_or("");
                                let source = line["source"].as_str().unwrap_or("");
                                format!("{:04}: {} | {}", addr, opcode, source)
                            })
                            .collect();
                        assembly_lines.set(lines);
                    }
                    cpu.set(cpu_mut);
                    error_message.set(None);
                }
                Err(e) => {
                    error_message.set(Some(format!("Assembly error: {:?}", e)));
                    assembly_lines.set(Vec::new());
                }
            }
        })
    };

    let on_step = {
        let cpu = cpu.clone();
        let error_message = error_message.clone();
        let last_acc = last_acc.clone();
        let last_ext = last_ext.clone();
        let last_iar = last_iar.clone();
        let last_xr1 = last_xr1.clone();
        let last_xr2 = last_xr2.clone();
        let last_xr3 = last_xr3.clone();
        let changed_memory = changed_memory.clone();

        Callback::from(move |_| {
            let mut cpu_mut = (*cpu).clone();

            // Save current state for change tracking
            let prev_xr1 = cpu_mut.get_xr1();
            let prev_xr2 = cpu_mut.get_xr2();
            let prev_xr3 = cpu_mut.get_xr3();

            last_acc.set(cpu_mut.get_acc());
            last_ext.set(cpu_mut.get_ext());
            last_iar.set(cpu_mut.get_iar());
            last_xr1.set(prev_xr1);
            last_xr2.set(prev_xr2);
            last_xr3.set(prev_xr3);

            let iar = cpu_mut.get_iar();

            match cpu_mut.read_memory(iar) {
                Ok(opcode) => {
                    if let Err(e) = cpu_mut.step(opcode) {
                        error_message.set(Some(format!("Execution error: {:?}", e)));
                    } else {
                        error_message.set(None);

                        // Track memory-mapped register changes (XR1@0x0001, XR2@0x0002, XR3@0x0003)
                        // IBM 1130 uses word addressing - XR1, XR2, XR3 are at word addresses 1, 2, 3
                        let mut changed = Vec::new();
                        if cpu_mut.get_xr1() != prev_xr1 {
                            changed.push(1); // XR1 at word address 1
                        }
                        if cpu_mut.get_xr2() != prev_xr2 {
                            changed.push(2); // XR2 at word address 2
                        }
                        if cpu_mut.get_xr3() != prev_xr3 {
                            changed.push(3); // XR3 at word address 3
                        }
                        changed_memory.set(changed);
                    }
                    cpu.set(cpu_mut);
                }
                Err(e) => {
                    error_message.set(Some(format!("Memory read error: {:?}", e)));
                }
            }
        })
    };

    let on_run = {
        let cpu = cpu.clone();
        let error_message = error_message.clone();
        Callback::from(move |_| {
            let mut cpu_mut = (*cpu).clone();
            match cpu_mut.run(10000) {
                Ok(_) => {
                    error_message.set(None);
                }
                Err(e) => {
                    error_message.set(Some(format!("Run error: {:?}", e)));
                }
            }
            cpu.set(cpu_mut);
        })
    };

    let on_reset = {
        let cpu = cpu.clone();
        let error_message = error_message.clone();
        let assembly_lines = assembly_lines.clone();
        let changed_memory = changed_memory.clone();
        Callback::from(move |_| {
            let mut cpu_mut = (*cpu).clone();
            cpu_mut.hard_reset();
            cpu.set(cpu_mut);
            error_message.set(None);
            assembly_lines.set(Vec::new());
            changed_memory.set(Vec::new());
        })
    };

    // Sidebar callbacks
    let on_tutorial = {
        let tutorial_open = tutorial_open.clone();
        Callback::from(move |_| tutorial_open.set(true))
    };

    let on_examples = {
        let examples_open = examples_open.clone();
        Callback::from(move |_| examples_open.set(true))
    };

    let on_challenges = {
        let challenges_open = challenges_open.clone();
        Callback::from(move |_| challenges_open.set(true))
    };

    let on_isa = {
        let isa_open = isa_open.clone();
        Callback::from(move |_| isa_open.set(true))
    };

    let on_help = {
        let help_open = help_open.clone();
        Callback::from(move |_| help_open.set(true))
    };

    // Modal close callbacks
    let close_tutorial = {
        let tutorial_open = tutorial_open.clone();
        Callback::from(move |_| tutorial_open.set(false))
    };

    let close_examples = {
        let examples_open = examples_open.clone();
        Callback::from(move |_| examples_open.set(false))
    };

    let close_challenges = {
        let challenges_open = challenges_open.clone();
        Callback::from(move |_| challenges_open.set(false))
    };

    let close_isa = {
        let isa_open = isa_open.clone();
        Callback::from(move |_| isa_open.set(false))
    };

    let close_help = {
        let help_open = help_open.clone();
        Callback::from(move |_| help_open.set(false))
    };

    // Load example callback
    let load_example = |example_code: &'static str| {
        let cpu = cpu.clone();
        let editor_code = editor_code.clone();
        let assembly_lines = assembly_lines.clone();
        let error_message = error_message.clone();
        let examples_open = examples_open.clone();

        Callback::from(move |_: MouseEvent| {
            // Reset CPU
            let mut cpu_mut = (*cpu).clone();
            cpu_mut.reset();
            cpu.set(cpu_mut);

            // Clear assembly output and errors
            assembly_lines.set(Vec::new());
            error_message.set(None);

            // Load code
            editor_code.set(example_code.to_string());
            examples_open.set(false);
        })
    };

    // Challenge callbacks
    let load_challenge = |challenge: Challenge| {
        let cpu = cpu.clone();
        let editor_code = editor_code.clone();
        let assembly_lines = assembly_lines.clone();
        let error_message = error_message.clone();
        let challenges_open = challenges_open.clone();
        let current_challenge = current_challenge.clone();

        Callback::from(move |_: MouseEvent| {
            // Reset CPU
            let mut cpu_mut = (*cpu).clone();
            cpu_mut.hard_reset();
            cpu.set(cpu_mut);

            // Clear assembly output and errors
            assembly_lines.set(Vec::new());
            error_message.set(None);

            // Load challenge template
            let mut code = format!(
                "; {}\n; {}\n\n",
                challenge.title,
                challenge.description.lines().next().unwrap_or("")
            );

            // Add initial data if present
            if !challenge.test_cases.is_empty()
                && !challenge.test_cases[0].initial_memory.is_empty() {
                    code.push_str("; Initial data:\n");
                    for (addr, value) in &challenge.test_cases[0].initial_memory {
                        code.push_str(&format!("DATA 0x{:02X} {}\n", addr, value));
                    }
                }

            editor_code.set(code);
            current_challenge.set(Some(challenge.clone()));
            challenges_open.set(false);
        })
    };

    // Check solution callback
    let check_solution = {
        let cpu = cpu.clone();
        let current_challenge = current_challenge.clone();
        let challenge_result = challenge_result.clone();

        Callback::from(move |_: MouseEvent| {
            if let Some(challenge) = (*current_challenge).as_ref() {
                match challenge.validate_solution((*cpu).cpu_state()) {
                    Ok(validation) => {
                        if validation.passed {
                            let mut message =
                                format!("✅ Challenge {} PASSED!\n\n", validation.challenge_id);
                            for test in &validation.test_results {
                                message.push_str(&format!("✓ {}\n", test.test_name));
                                message.push_str(&format!(
                                    "  Cycles: {}, Instructions: {}\n",
                                    test.cycles, test.instructions
                                ));
                            }
                            challenge_result.set(Some(Ok(message)));

                            // Save to localStorage
                            if let Some(window) = web_sys::window()
                                && let Ok(Some(storage)) = window.local_storage() {
                                    let key =
                                        format!("ibm1130_challenge_{}", validation.challenge_id);
                                    let _ = storage.set_item(&key, "completed");
                                }
                        } else {
                            let mut message = format!(
                                "❌ Challenge {} did not pass.\n\n",
                                validation.challenge_id
                            );
                            for test in &validation.test_results {
                                if test.passed {
                                    message.push_str(&format!("✓ {}\n", test.test_name));
                                } else {
                                    message.push_str(&format!("✗ {}\n", test.test_name));
                                    if let Some(error) = &test.error {
                                        message.push_str(&format!("  Error: {}\n", error));
                                    }
                                }
                            }
                            challenge_result.set(Some(Err(message)));
                        }
                    }
                    Err(e) => {
                        challenge_result.set(Some(Err(format!("❌ Validation error: {}", e))));
                    }
                }
            }
        })
    };

    // Dismiss challenge result
    let dismiss_result = {
        let challenge_result = challenge_result.clone();
        Callback::from(move |_| {
            challenge_result.set(None);
        })
    };

    // Tab change callback
    let on_tab_change = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: Tab| {
            active_tab.set(tab);
        })
    };

    // Keypunch: Load deck into assembler editor
    let load_deck_to_editor = {
        let keypunch_deck = keypunch_deck.clone();
        let editor_code = editor_code.clone();
        let active_tab = active_tab.clone();
        Callback::from(move |_: MouseEvent| {
            // Convert deck cards to assembly source
            let code: String = keypunch_deck.cards.iter()
                .map(|card| card.to_text().trim_end().to_string())
                .collect::<Vec<_>>()
                .join("\n");
            editor_code.set(code);
            active_tab.set(Tab::Assembler);
        })
    };

    // Assembler: Send listing to printer
    let send_to_printer = {
        let assembly_lines = assembly_lines.clone();
        let printer_content = printer_content.clone();
        let active_tab = active_tab.clone();
        Callback::from(move |_: MouseEvent| {
            if !assembly_lines.is_empty() {
                printer_content.set((*assembly_lines).clone());
                active_tab.set(Tab::Printer);
            }
        })
    };

    // Build console registers from CPU state
    let build_console_registers = |cpu_state: &Option<serde_json::Value>| -> ConsoleRegisters {
        if let Some(state) = cpu_state {
            ConsoleRegisters {
                acc: state["acc"].as_u64().unwrap_or(0) as u16,
                ext: state["ext"].as_u64().unwrap_or(0) as u16,
                iar: state["iar"].as_u64().unwrap_or(0) as u16,
                sar: 0, // SAR not exposed in current CPU model
                sbr: 0, // SBR not exposed in current CPU model
                afr: 0, // AFR not exposed in current CPU model
            }
        } else {
            ConsoleRegisters::default()
        }
    };

    // Get CPU state for display
    let cpu_state = cpu
        .get_state()
        .ok()
        .and_then(|js_val| serde_wasm_bindgen::from_value::<serde_json::Value>(js_val).ok());

    // Build register list for RegisterPanel
    let mut registers = Vec::new();
    if let Some(state) = &cpu_state {
        let acc = state["acc"].as_u64().unwrap_or(0) as u16;
        let ext = state["ext"].as_u64().unwrap_or(0) as u16;
        let iar = state["iar"].as_u64().unwrap_or(0) as u16;
        let xr1 = state["xr1"].as_u64().unwrap_or(0) as u16;
        let xr2 = state["xr2"].as_u64().unwrap_or(0) as u16;
        let xr3 = state["xr3"].as_u64().unwrap_or(0) as u16;

        registers.push(Register {
            name: "ACC".to_string(),
            value: format!("0x{:04X} ({})", acc, acc as i16),
            changed: acc != *last_acc,
        });
        registers.push(Register {
            name: "EXT".to_string(),
            value: format!("0x{:04X} ({})", ext, ext as i16),
            changed: ext != *last_ext,
        });
        registers.push(Register {
            name: "IAR".to_string(),
            value: format!("0x{:04X} ({})", iar, iar),
            changed: iar != *last_iar,
        });
        registers.push(Register {
            name: "XR1".to_string(),
            value: format!("0x{:04X} ({})", xr1, xr1),
            changed: xr1 != *last_xr1,
        });
        registers.push(Register {
            name: "XR2".to_string(),
            value: format!("0x{:04X} ({})", xr2, xr2),
            changed: xr2 != *last_xr2,
        });
        registers.push(Register {
            name: "XR3".to_string(),
            value: format!("0x{:04X} ({})", xr3, xr3),
            changed: xr3 != *last_xr3,
        });
    }

    // Build register legend
    let legend_items = vec![
        LegendItem {
            label: "ACC".to_string(),
            value: "Accumulator".to_string(),
            changed: false,
        },
        LegendItem {
            label: "EXT".to_string(),
            value: "Extension".to_string(),
            changed: false,
        },
        LegendItem {
            label: "IAR".to_string(),
            value: "Instr Addr Reg".to_string(),
            changed: false,
        },
        LegendItem {
            label: "XR1".to_string(),
            value: "Index Reg 1".to_string(),
            changed: false,
        },
        LegendItem {
            label: "XR2".to_string(),
            value: "Index Reg 2".to_string(),
            changed: false,
        },
        LegendItem {
            label: "XR3".to_string(),
            value: "Index Reg 3".to_string(),
            changed: false,
        },
    ];

    // Build flags display
    let flags_html = if let Some(state) = &cpu_state {
        let carry = state["carry"].as_bool().unwrap_or(false);
        let overflow = state["overflow"].as_bool().unwrap_or(false);
        let acc = state["acc"].as_u64().unwrap_or(0) as u16;
        let positive = (acc & 0x8000) == 0 && acc != 0;
        let zero = acc == 0;

        html! {
            <div class="flags">
                <div class="flag">
                    <div class={if carry { "flag-indicator set" } else { "flag-indicator" }}></div>
                    <span>{"C (Carry)"}</span>
                </div>
                <div class="flag">
                    <div class={if overflow { "flag-indicator set" } else { "flag-indicator" }}></div>
                    <span>{"V (Overflow)"}</span>
                </div>
                <div class="flag">
                    <div class={if positive { "flag-indicator set" } else { "flag-indicator" }}></div>
                    <span>{"P (Positive)"}</span>
                </div>
                <div class="flag">
                    <div class={if zero { "flag-indicator set" } else { "flag-indicator" }}></div>
                    <span>{"Z (Zero)"}</span>
                </div>
            </div>
        }
    } else {
        html! {}
    };

    // Build CPU status display
    let status_html = if let Some(state) = &cpu_state {
        let cycles = state["cycle_count"].as_u64().unwrap_or(0);
        let instructions = state["instruction_count"].as_u64().unwrap_or(0);
        let halted = state["halted"].as_bool().unwrap_or(false);
        let status_text = if halted { "Halted" } else { "Ready" };

        html! {
            <div class="cpu-status">
                <div class="status-item">
                    <span class="status-label">{"Cycles:"}</span>
                    <span class="status-value">{cycles}</span>
                </div>
                <div class="status-item">
                    <span class="status-label">{"Instructions:"}</span>
                    <span class="status-value">{instructions}</span>
                </div>
                <div class="status-item">
                    <span class="status-label">{"Status:"}</span>
                    <span class="status-value">{status_text}</span>
                </div>
            </div>
        }
    } else {
        html! {}
    };

    // Build memory viewer - IBM 1130 uses 16-bit word addressing
    // Display memory as words with word addresses for clarity
    let memory_words = if let Some(state) = &cpu_state {
        if let Some(memory_array) = state["memory"].as_array() {
            memory_array
                .iter()
                .map(|v| v.as_u64().unwrap_or(0) as u16)
                .collect::<Vec<u16>>()
        } else {
            vec![0; 4096] // 4K words
        }
    } else {
        vec![0; 4096] // 4K words
    };

    // IAR contains word address - use directly for word-based memory viewer
    let pc = cpu_state
        .as_ref()
        .and_then(|s| s["iar"].as_u64())
        .map(|word_addr| word_addr as u16)
        .unwrap_or(0);

    // Sidebar buttons
    let sidebar_buttons = vec![
        SidebarButton {
            emoji: "📚".to_string(),
            label: "Tutorial".to_string(),
            onclick: on_tutorial,
            title: None,
        },
        SidebarButton {
            emoji: "📝".to_string(),
            label: "Examples".to_string(),
            onclick: on_examples,
            title: None,
        },
        SidebarButton {
            emoji: "🎯".to_string(),
            label: "Challenges".to_string(),
            onclick: on_challenges,
            title: None,
        },
        SidebarButton {
            emoji: "📖".to_string(),
            label: "ISA Reference".to_string(),
            onclick: on_isa,
            title: None,
        },
        SidebarButton {
            emoji: "❓".to_string(),
            label: "Help".to_string(),
            onclick: on_help,
            title: None,
        },
    ];

    // === TAB CONTENTS ===

    // Keypunch Tab Content
    let keypunch_content_html = html! {
        <div class="keypunch-tab">
            <Keypunch />
            <div class="keypunch-actions">
                <button class="load-to-assembler-btn" onclick={load_deck_to_editor.clone()}>
                    {"Load Deck → Assembler"}
                </button>
            </div>
        </div>
    };

    // Printer Tab Content
    let printer_content_html = html! {
        <div class="printer-tab">
            <Printer
                content={(*printer_content).clone()}
                auto_start={false}
                sound_enabled={true}
            />
        </div>
    };

    // Console Panel Tab Content
    let console_content_html = {
        let console_registers = build_console_registers(&cpu_state);
        html! {
            <div class="console-tab">
                <ConsolePanel external_registers={Some(console_registers)} />
            </div>
        }
    };

    html! {
        <div class="container">
            <Header
                title="IBM 1130 System Emulator"
                subtitle="Keypunch, Printer, Assembler, and Console"
            >
                <TabNav active_tab={*active_tab} on_tab_change={on_tab_change.clone()} />
            </Header>

            <TabContainer
                active_tab={*active_tab}
                on_tab_change={on_tab_change}
                keypunch_content={keypunch_content_html}
                printer_content={printer_content_html}
                console_content={console_content_html}
                assembler_content={html! {
                    <div class="assembler-tab">

            <Sidebar buttons={sidebar_buttons} />

            <div class="main-content">
                // Left Column: Input + Controls + Output
                <div class="left-column">
                    // Input Panel (scrollable)
                    <div class="input-panel">
                        <div class="panel-title">{"Program Editor"}</div>
                        <ProgramArea
                            initial_code={Some((*editor_code).clone())}
                            assembly_output={None}
                            on_assemble={on_assemble}
                            on_step={on_step}
                            on_run={on_run}
                            on_reset={on_reset}
                            step_enabled={!cpu.is_halted() && !assembly_lines.is_empty()}
                            run_enabled={!cpu.is_halted() && !assembly_lines.is_empty()}
                        />
                    </div>

                    // Output Panel (scrollable)
                    <div class="output-panel">
                        <div class="panel-title">{"Assembly Output"}</div>
                        <div class="assembly-output">
                            {if assembly_lines.is_empty() {
                                html! { <div class="empty-state">{"Click 'Assemble' to see output"}</div> }
                            } else {
                                let pc = cpu.get_iar();
                                html! {
                                    <div>
                                        {for assembly_lines.iter().map(|line| {
                                            let addr_str = line.split(':').next().unwrap_or("");
                                            let is_current = if let Ok(addr) = addr_str.parse::<u16>() {
                                                addr == pc
                                            } else {
                                                false
                                            };
                                            let class = if is_current { "assembly-line current" } else { "assembly-line" };
                                            html! { <div class={class}>{line}</div> }
                                        })}
                                    </div>
                                }
                            }}
                        </div>
                        <div class="integration-toolbar">
                            <button
                                class="send-to-printer-btn"
                                onclick={send_to_printer}
                                disabled={assembly_lines.is_empty()}
                            >
                                {"Send Listing → Printer"}
                            </button>
                        </div>
                    </div>
                </div>

                // Right Column: Registers + Memory
                <div class="right-column">
                    // Registers Section (compact, ~25%)
                    <div class="registers-section">
                        <RegisterPanel
                            registers={registers}
                            legend_items={legend_items}
                        />
                        {flags_html}
                        {status_html}
                    </div>

                    // Memory Section (scrollable, ~75%)
                    <div class="memory-section">
                        <WordMemoryViewer
                            memory={memory_words}
                            pc={pc}
                            title={Some("Memory (4K Words)".to_string())}
                            words_per_row={8}
                            words_to_show={4096}
                            changed_addresses={(*changed_memory).clone()}
                        />
                    </div>
                </div>
            </div>

            // Challenge Banner
            {if let Some(challenge) = (*current_challenge).as_ref() {
                html! {
                    <div class="challenge-banner">
                        <div class="challenge-info">
                            <strong>{&challenge.title}</strong>
                        </div>
                        <button class="check-solution-btn" onclick={check_solution}>
                            {"Check Solution"}
                        </button>
                    </div>
                }
            } else {
                html! {}
            }}

            // Challenge Result Banner
            {if let Some(result) = (*challenge_result).as_ref() {
                match result {
                    Ok(message) => html! {
                        <div class="success-banner">
                            <div class="banner-content">
                                <pre style="margin: 0; white-space: pre-wrap; font-family: inherit;">{message}</pre>
                            </div>
                            <button class="dismiss-btn" onclick={dismiss_result.clone()}>
                                {"×"}
                            </button>
                        </div>
                    },
                    Err(message) => html! {
                        <div class="error-banner">
                            <div class="banner-content">
                                <pre style="margin: 0; white-space: pre-wrap; font-family: inherit;">{message}</pre>
                            </div>
                            <button class="dismiss-btn" onclick={dismiss_result.clone()}>
                                {"×"}
                            </button>
                        </div>
                    }
                }
            } else {
                html! {}
            }}

            // Modals
            <Modal id="tutorial" title="Tutorial" active={*tutorial_open} on_close={close_tutorial}>
                <h3>{"Welcome to the IBM 1130 Assembly Game!"}</h3>
                <p>{"This interactive tool helps you learn the IBM 1130 instruction set architecture, a 16-bit minicomputer from 1965."}</p>

                <h3>{"IBM 1130 Architecture"}</h3>
                <p>{"The IBM 1130 is a word-addressed machine with:"}</p>
                <ul>
                    <li><strong>{"ACC"}</strong>{" - Accumulator: main register for arithmetic"}</li>
                    <li><strong>{"EXT"}</strong>{" - Extension register: for double-precision operations"}</li>
                    <li><strong>{"IAR"}</strong>{" - Instruction Address Register: program counter"}</li>
                    <li><strong>{"XR1-XR3"}</strong>{" - Index Registers: for indexed addressing (memory-mapped)"}</li>
                    <li><strong>{"Flags"}</strong>{" - C (Carry), V (Overflow), P (Positive), Z (Zero)"}</li>
                </ul>

                <h3>{"Getting Started"}</h3>
                <ol>
                    <li>{"Write your assembly program in the editor"}</li>
                    <li>{"Click "}<strong>{"Assemble"}</strong>{" to convert it to machine code"}</li>
                    <li>{"Use "}<strong>{"Step"}</strong>{" to execute one instruction at a time"}</li>
                    <li>{"Use "}<strong>{"Run"}</strong>{" to execute until WAIT instruction"}</li>
                    <li>{"Watch registers and memory update in real-time!"}</li>
                </ol>

                <h3>{"Assembly Syntax"}</h3>
                <p>{"Instructions follow this format:"}</p>
                <pre>{"MNEMONIC mode address   ; comment"}</pre>
                <p>{"Example:"}</p>
                <pre>{"LD 0 10    ; Load from address 10 (direct mode)\nA 1 20     ; Add from address 20+XR1 (indexed mode)"}</pre>
                <p>{"Data directives:"}</p>
                <pre>{"DATA address value   ; Store value at address"}</pre>
            </Modal>

            <Modal id="examples" title="Example Programs" active={*examples_open} on_close={close_examples}>
                <div class="example-item" onclick={load_example(example_1)}>
                    <h4>{"Example 1: Add Two Numbers"}</h4>
                    <p>{"Basic arithmetic: load two numbers, add them, and store the result"}</p>
                </div>

                <div class="example-item" onclick={load_example(example_2)}>
                    <h4>{"Example 2: Indexed Addressing"}</h4>
                    <p>{"Use index register XR1 to access array elements"}</p>
                </div>

                <div class="example-item" onclick={load_example(example_3)}>
                    <h4>{"Example 3: Shift Operations"}</h4>
                    <p>{"Shift left to multiply by 4 (efficient multiplication)"}</p>
                </div>

                <div class="example-item" onclick={load_example(example_4)}>
                    <h4>{"Example 4: Memory-Mapped Registers"}</h4>
                    <p>{"Demonstrates XR1-XR3 as memory locations with highlighting"}</p>
                </div>

                <div class="example-item" onclick={load_example(example_5)}>
                    <h4>{"Example 5: Special Memory Locations"}</h4>
                    <p>{"Learn about location 0 (safety trap), locations 1-3 (index registers), and interrupt vectors"}</p>
                </div>
            </Modal>

            <Modal id="challenges" title="Challenges" active={*challenges_open} on_close={close_challenges}>
                {for challenges.iter().map(|challenge| {
                    let difficulty_color = match challenge.difficulty {
                        crate::challenge::Difficulty::Beginner => "#4caf50",
                        crate::challenge::Difficulty::Intermediate => "#ff9800",
                        crate::challenge::Difficulty::Advanced => "#f44336",
                    };

                    html! {
                        <div class="challenge-item" onclick={load_challenge(challenge.clone())} style={format!("border-left: 4px solid {}", difficulty_color)}>
                            <h4>{&challenge.title}</h4>
                            <p>{challenge.description.lines().next().unwrap_or("")}</p>
                            {if !challenge.hints.is_empty() {
                                html! {
                                    <details style="margin-top: 8px;">
                                        <summary style="cursor: pointer; color: #00d9ff;">{"💡 Hints"}</summary>
                                        <ul style="margin: 5px 0; padding-left: 20px;">
                                            {for challenge.hints.iter().map(|hint| {
                                                html! { <li style="color: #aaa; margin: 5px 0;">{hint}</li> }
                                            })}
                                        </ul>
                                    </details>
                                }
                            } else {
                                html! {}
                            }}
                        </div>
                    }
                })}
            </Modal>

            <Modal id="isa" title="IBM 1130 ISA Reference" active={*isa_open} on_close={close_isa}>
                <h3>{"Instruction Set"}</h3>
                <table class="instruction-table">
                    <tr>
                        <th>{"Instruction"}</th>
                        <th>{"Description"}</th>
                        <th>{"Format"}</th>
                    </tr>
                    <tr>
                        <td><code>{"LD mode addr"}</code></td>
                        <td>{"Load ACC from memory"}</td>
                        <td>{"ACC ← memory[addr]"}</td>
                    </tr>
                    <tr>
                        <td><code>{"STO mode addr"}</code></td>
                        <td>{"Store ACC to memory"}</td>
                        <td>{"memory[addr] ← ACC"}</td>
                    </tr>
                    <tr>
                        <td><code>{"LDX addr"}</code></td>
                        <td>{"Load XR1 from memory"}</td>
                        <td>{"XR1 ← memory[addr]"}</td>
                    </tr>
                    <tr>
                        <td><code>{"STX addr"}</code></td>
                        <td>{"Store XR1 to memory"}</td>
                        <td>{"memory[addr] ← XR1"}</td>
                    </tr>
                    <tr>
                        <td><code>{"A mode addr"}</code></td>
                        <td>{"Add to ACC"}</td>
                        <td>{"ACC ← ACC + memory[addr]"}</td>
                    </tr>
                    <tr>
                        <td><code>{"S mode addr"}</code></td>
                        <td>{"Subtract from ACC"}</td>
                        <td>{"ACC ← ACC - memory[addr]"}</td>
                    </tr>
                    <tr>
                        <td><code>{"AND mode addr"}</code></td>
                        <td>{"Logical AND with ACC"}</td>
                        <td>{"ACC ← ACC & memory[addr]"}</td>
                    </tr>
                    <tr>
                        <td><code>{"OR mode addr"}</code></td>
                        <td>{"Logical OR with ACC"}</td>
                        <td>{"ACC ← ACC | memory[addr]"}</td>
                    </tr>
                    <tr>
                        <td><code>{"SLA count"}</code></td>
                        <td>{"Shift left ACC"}</td>
                        <td>{"ACC ← ACC << count"}</td>
                    </tr>
                    <tr>
                        <td><code>{"SRA count"}</code></td>
                        <td>{"Shift right ACC (arithmetic)"}</td>
                        <td>{"ACC ← ACC >> count"}</td>
                    </tr>
                    <tr>
                        <td><code>{"BSC cond addr"}</code></td>
                        <td>{"Branch on condition"}</td>
                        <td>{"if condition then IAR ← addr"}</td>
                    </tr>
                    <tr>
                        <td><code>{"BSI addr"}</code></td>
                        <td>{"Branch and store IAR"}</td>
                        <td>{"memory[addr] ← IAR; IAR ← addr+1"}</td>
                    </tr>
                    <tr>
                        <td><code>{"WAIT"}</code></td>
                        <td>{"Halt execution"}</td>
                        <td>{"Stop CPU"}</td>
                    </tr>
                    <tr>
                        <td><code>{"NOP"}</code></td>
                        <td>{"No operation"}</td>
                        <td>{"Do nothing"}</td>
                    </tr>
                </table>

                <h3>{"Addressing Modes"}</h3>
                <ul>
                    <li><code>{"0"}</code>{" - Direct: use address as-is"}</li>
                    <li><code>{"1"}</code>{" - Indexed: add XR1 to address"}</li>
                </ul>

                <h3>{"Branch Conditions"}</h3>
                <ul>
                    <li><code>{"Z"}</code>{" - Zero: ACC == 0"}</li>
                    <li><code>{"NZ"}</code>{" - Non-zero: ACC != 0"}</li>
                    <li><code>{"P"}</code>{" - Positive: ACC > 0"}</li>
                    <li><code>{"N"}</code>{" - Negative: ACC < 0"}</li>
                    <li><code>{"C"}</code>{" - Carry flag set"}</li>
                    <li><code>{"V"}</code>{" - Overflow flag set"}</li>
                </ul>

                <h3>{"Flags"}</h3>
                <ul>
                    <li><strong>{"C (Carry)"}</strong>{" - Set on unsigned overflow"}</li>
                    <li><strong>{"V (Overflow)"}</strong>{" - Set on signed overflow"}</li>
                    <li><strong>{"P (Positive)"}</strong>{" - Set when ACC > 0"}</li>
                    <li><strong>{"Z (Zero)"}</strong>{" - Set when ACC == 0"}</li>
                </ul>
            </Modal>

            <Modal id="help" title="Help" active={*help_open} on_close={close_help}>
                <h3>{"Controls"}</h3>
                <ul>
                    <li><strong>{"Assemble"}</strong>{" - Convert assembly code to machine code"}</li>
                    <li><strong>{"Step"}</strong>{" - Execute one instruction"}</li>
                    <li><strong>{"Run"}</strong>{" - Execute until WAIT or error"}</li>
                    <li><strong>{"Reset"}</strong>{" - Clear CPU state (keep program)"}</li>
                    <li><strong>{"Check"}</strong>{" - Validate challenge solution"}</li>
                </ul>

                <h3>{"Tips"}</h3>
                <ul>
                    <li>{"Comments start with semicolon (;)"}</li>
                    <li>{"Use DATA directive to initialize memory"}</li>
                    <li>{"Watch register changes highlighted in green"}</li>
                    <li>{"Current instruction highlighted in cyan"}</li>
                    <li>{"Memory-mapped registers (XR1-XR3) are underlined in memory viewer"}</li>
                </ul>

                <h3>{"About"}</h3>
                <p>{"This educational tool simulates a simplified IBM 1130 minicomputer."}</p>
                <p>{"The IBM 1130 was introduced in 1965 and was widely used in scientific and educational institutions."}</p>
            </Modal>

                    </div> // End assembler-tab
                }}
            />

            // GitHub Corner
            <a href="https://github.com/sw-comp-history/ibm-1130-rs" class="github-corner" aria-label="View source on GitHub" target="_blank" rel="noopener">
                <svg viewBox="0 0 250 250" aria-hidden="true">
                    <path d="M0,0 L115,115 L130,115 L142,142 L250,250 L250,0 Z"></path>
                    <path d="M128.3,109.0 C113.8,99.7 119.0,89.6 119.0,89.6 C122.0,82.7 120.5,78.6 120.5,78.6 C119.2,72.0 123.4,76.3 123.4,76.3 C127.3,80.9 125.5,87.3 125.5,87.3 C122.9,97.6 130.6,101.9 134.4,103.2" fill="currentColor" style="transform-origin: 130px 106px;" class="octo-arm"></path>
                    <path d="M115.0,115.0 C114.9,115.1 118.7,116.5 119.8,115.4 L133.7,101.6 C136.9,99.2 139.9,98.4 142.2,98.6 C133.8,88.0 127.5,74.4 143.8,58.0 C148.5,53.4 154.0,51.2 159.7,51.0 C160.3,49.4 163.2,43.6 171.4,40.1 C171.4,40.1 176.1,42.5 178.8,56.2 C183.1,58.6 187.2,61.8 190.9,65.4 C194.5,69.0 197.7,73.2 200.1,77.6 C213.8,80.2 216.3,84.9 216.3,84.9 C212.7,93.1 206.9,96.0 205.4,96.6 C205.1,102.4 203.0,107.8 198.3,112.5 C181.9,128.9 168.3,122.5 157.7,114.1 C157.9,116.9 156.7,120.9 152.7,124.9 L141.0,136.5 C139.8,137.7 141.6,141.9 141.8,141.8 Z" fill="currentColor" class="octo-body"></path>
                </svg>
            </a>

            // Footer
            <footer class="app-footer">
                <div class="footer-left">
                    <span>{"MIT License"}</span>
                    <span>{"© 2026 Michael A Wright"}</span>
                </div>
                <div class="footer-right">
                    <span>{format!("{} | {} | {}", env!("VERGEN_BUILD_HOST"), env!("VERGEN_GIT_SHA_SHORT"), env!("VERGEN_BUILD_TIMESTAMP"))}</span>
                </div>
            </footer>
        </div>
    }
}
