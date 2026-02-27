// Keypunch Component
//
// IBM 029 Keypunch simulator with:
// - Visual punch card with 80 columns × 12 rows
// - Keyboard input → hole punches
// - Multiple cards in a deck (stack visualization)
// - Save/Load deck as file

use yew::prelude::*;
use punch_card_core::punch_card::{CardType, PunchCard};
use gloo::file::{Blob, ObjectUrl};
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;

/// Represents a deck of punch cards
#[derive(Clone, PartialEq)]
pub struct Deck {
    pub cards: Vec<PunchCard>,
    pub current_card: usize,
}

impl Default for Deck {
    fn default() -> Self {
        Self {
            cards: vec![PunchCard::new(CardType::Text)],
            current_card: 0,
        }
    }
}

impl Deck {
    /// Get the current card
    pub fn current(&self) -> &PunchCard {
        &self.cards[self.current_card]
    }

    /// Get mutable reference to current card
    pub fn current_mut(&mut self) -> &mut PunchCard {
        &mut self.cards[self.current_card]
    }

    /// Add a new blank card to the deck
    pub fn add_card(&mut self) {
        self.cards.push(PunchCard::new(CardType::Text));
        self.current_card = self.cards.len() - 1;
    }

    /// Navigate to next card
    pub fn next_card(&mut self) {
        if self.current_card < self.cards.len() - 1 {
            self.current_card += 1;
        }
    }

    /// Navigate to previous card
    pub fn prev_card(&mut self) {
        if self.current_card > 0 {
            self.current_card -= 1;
        }
    }

    /// Delete the current card
    pub fn delete_card(&mut self) {
        if self.cards.len() > 1 {
            self.cards.remove(self.current_card);
            if self.current_card >= self.cards.len() {
                self.current_card = self.cards.len() - 1;
            }
        }
    }

    /// Convert deck to binary data for saving (108 bytes per card)
    pub fn to_binary(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for card in &self.cards {
            data.extend(card.to_binary());
        }
        data
    }

    /// Load deck from binary data (108 bytes per card)
    pub fn from_binary(data: &[u8]) -> Self {
        let card_size = 108;
        let mut cards = Vec::new();

        for chunk in data.chunks(card_size) {
            if chunk.len() == card_size {
                cards.push(PunchCard::from_binary(chunk));
            }
        }

        if cards.is_empty() {
            return Self::default();
        }

        Self {
            cards,
            current_card: 0,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct KeypunchProps {
    /// Callback when deck changes
    #[prop_or_default]
    pub on_deck_change: Callback<Deck>,
}

#[function_component(Keypunch)]
pub fn keypunch(props: &KeypunchProps) -> Html {
    let deck = use_state(Deck::default);
    let current_column = use_state(|| 0usize);
    let download_url = use_state(|| None::<ObjectUrl>);

    // Handle keyboard input
    let on_key_press = {
        let deck = deck.clone();
        let current_column = current_column.clone();
        let on_deck_change = props.on_deck_change.clone();
        Callback::from(move |e: KeyboardEvent| {
            let key = e.key();

            // Handle special keys
            match key.as_str() {
                "Enter" => {
                    // Move to next card
                    let mut new_deck = (*deck).clone();
                    new_deck.add_card();
                    deck.set(new_deck.clone());
                    current_column.set(0);
                    on_deck_change.emit(new_deck);
                    return;
                }
                "Backspace" => {
                    // Move back one column and clear
                    if *current_column > 0 {
                        let col = *current_column - 1;
                        let mut new_deck = (*deck).clone();
                        let _ = new_deck.current_mut().clear_column(col);
                        deck.set(new_deck.clone());
                        current_column.set(col);
                        on_deck_change.emit(new_deck);
                    }
                    return;
                }
                "Tab" => {
                    e.prevent_default();
                    // Skip to next field (every 10 columns)
                    let next_field = ((*current_column / 10) + 1) * 10;
                    if next_field < 80 {
                        current_column.set(next_field);
                    }
                    return;
                }
                _ => {}
            }

            // Handle printable character
            if key.len() == 1 && *current_column < 80
                && let Some(c) = key.chars().next() {
                    let mut new_deck = (*deck).clone();
                    let _ = new_deck.current_mut().set_column_char(*current_column, c);
                    deck.set(new_deck.clone());
                    current_column.set(*current_column + 1);
                    on_deck_change.emit(new_deck);
                }
        })
    };

    // Handle text input change
    let on_text_input = {
        let deck = deck.clone();
        let current_column = current_column.clone();
        let on_deck_change = props.on_deck_change.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target()
                && let Ok(input) = input.dyn_into::<HtmlInputElement>() {
                    let text = input.value();
                    let mut new_deck = (*deck).clone();
                    let card = new_deck.current_mut();

                    // Clear current card and repunch
                    card.clear();
                    for (i, c) in text.chars().take(80).enumerate() {
                        let _ = card.set_column_char(i, c);
                    }

                    let col = text.len().min(80);
                    deck.set(new_deck.clone());
                    current_column.set(col);
                    on_deck_change.emit(new_deck);
                }
        })
    };

    // Navigation handlers
    let on_prev_card = {
        let deck = deck.clone();
        let current_column = current_column.clone();
        Callback::from(move |_: MouseEvent| {
            let mut new_deck = (*deck).clone();
            new_deck.prev_card();
            deck.set(new_deck);
            current_column.set(0);
        })
    };

    let on_next_card = {
        let deck = deck.clone();
        let current_column = current_column.clone();
        Callback::from(move |_: MouseEvent| {
            let mut new_deck = (*deck).clone();
            new_deck.next_card();
            deck.set(new_deck);
            current_column.set(0);
        })
    };

    let on_new_card = {
        let deck = deck.clone();
        let current_column = current_column.clone();
        let on_deck_change = props.on_deck_change.clone();
        Callback::from(move |_: MouseEvent| {
            let mut new_deck = (*deck).clone();
            new_deck.add_card();
            deck.set(new_deck.clone());
            current_column.set(0);
            on_deck_change.emit(new_deck);
        })
    };

    let on_clear_card = {
        let deck = deck.clone();
        let current_column = current_column.clone();
        let on_deck_change = props.on_deck_change.clone();
        Callback::from(move |_: MouseEvent| {
            let mut new_deck = (*deck).clone();
            new_deck.current_mut().clear();
            deck.set(new_deck.clone());
            current_column.set(0);
            on_deck_change.emit(new_deck);
        })
    };

    // Save deck handler
    let on_save = {
        let deck = deck.clone();
        let download_url = download_url.clone();
        Callback::from(move |_: MouseEvent| {
            let binary = deck.to_binary();
            let blob = Blob::new_with_options(&binary[..], Some("application/octet-stream"));
            let url = ObjectUrl::from(blob);
            download_url.set(Some(url));
        })
    };

    // Load deck handler
    let on_load = {
        let deck = deck.clone();
        let current_column = current_column.clone();
        let on_deck_change = props.on_deck_change.clone();
        Callback::from(move |e: Event| {
            let deck = deck.clone();
            let current_column = current_column.clone();
            let on_deck_change = on_deck_change.clone();

            if let Some(input) = e.target()
                && let Ok(input) = input.dyn_into::<HtmlInputElement>()
                    && let Some(files) = input.files()
                        && let Some(file) = files.get(0) {
                            let file = gloo::file::File::from(file);
                            let reader = gloo::file::callbacks::read_as_bytes(&file, move |result| {
                                if let Ok(data) = result {
                                    let new_deck = Deck::from_binary(&data);
                                    deck.set(new_deck.clone());
                                    current_column.set(0);
                                    on_deck_change.emit(new_deck);
                                }
                            });
                            // Keep the reader alive
                            std::mem::forget(reader);
                        }
        })
    };

    // Get current card text for input display
    let current_text = deck.current().to_text().trim_end().to_string();

    html! {
        <div class="keypunch" tabindex="0" onkeydown={on_key_press}>
            <div class="keypunch-header">
                <h2 class="keypunch-title">{"IBM 029 Keypunch"}</h2>
                <div class="deck-info">
                    <span class="card-count">
                        {format!("Card {} of {}", deck.current_card + 1, deck.cards.len())}
                    </span>
                    <span class="column-info">
                        {format!("Column: {}", *current_column + 1)}
                    </span>
                </div>
            </div>

            // Text input for typing
            <div class="keypunch-input">
                <label for="card-text">{"Type card content:"}</label>
                <input
                    id="card-text"
                    type="text"
                    value={current_text}
                    maxlength="80"
                    oninput={on_text_input}
                    placeholder="Type up to 80 characters..."
                />
            </div>

            // Punch card display
            <div class="punch-card-display">
                <PunchCardSvg
                    card={deck.current().clone()}
                    current_column={Some(*current_column)}
                />
            </div>

            // Controls
            <div class="keypunch-controls">
                <div class="nav-buttons">
                    <button onclick={on_prev_card} disabled={deck.current_card == 0}>
                        {"< Prev"}
                    </button>
                    <button onclick={on_next_card} disabled={deck.current_card >= deck.cards.len() - 1}>
                        {"Next >"}
                    </button>
                    <button onclick={on_new_card}>{"New Card"}</button>
                    <button onclick={on_clear_card}>{"Clear Card"}</button>
                </div>
                <div class="file-buttons">
                    <button onclick={on_save}>{"Save Deck"}</button>
                    if let Some(url) = &*download_url {
                        <a href={url.to_string()} download="deck.bin" class="download-link">
                            {"Download"}
                        </a>
                    }
                    <label class="file-input-label">
                        {"Load Deck"}
                        <input type="file" accept=".bin" onchange={on_load} />
                    </label>
                </div>
            </div>

            // Deck preview (card stack)
            <div class="deck-preview">
                { for deck.cards.iter().enumerate().map(|(idx, card)| {
                    let is_current = idx == deck.current_card;
                    let preview_text: String = card.to_text().chars().take(20).collect();
                    let class = if is_current { "card-preview current" } else { "card-preview" };

                    html! {
                        <div class={class}>
                            <span class="preview-number">{idx + 1}</span>
                            <span class="preview-text">{preview_text}</span>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}

/// SVG Punch Card Component
#[derive(Properties, PartialEq)]
pub struct PunchCardSvgProps {
    pub card: PunchCard,
    #[prop_or(None)]
    pub current_column: Option<usize>,
}

#[function_component(PunchCardSvg)]
pub fn punch_card_svg(props: &PunchCardSvgProps) -> Html {
    let card = &props.card;
    let current_col = props.current_column;

    // SVG dimensions - proper IBM card aspect ratio (7⅜" × 3¼")
    let card_width = 800.0;
    let card_height = card_width / 2.269;

    // Margins
    let left_margin = card_width * 0.025;
    let right_margin = card_width * 0.025;
    let top_margin = card_height * 0.045;

    // Punch area dimensions
    let punch_width_area = card_width - left_margin - right_margin;
    let punch_height_area = card_height - top_margin * 2.0;

    let col_width = punch_width_area / 80.0;
    let row_height = punch_height_area / 12.0;
    let text_y = top_margin - 5.0;
    let grid_start_y = top_margin;

    let punch_width = col_width * 0.6;
    let punch_height = row_height * 0.7;
    let guide_width = col_width * 0.5;
    let guide_height = row_height * 0.6;

    html! {
        <div class="punch-card-container">
            <svg class="punch-card" viewBox={format!("0 0 {} {}", card_width, card_height)} xmlns="http://www.w3.org/2000/svg">
                // Card background with corner cut
                <polygon
                    points={format!("{},{} {},{} {},{} {},{} {},{}",
                        left_margin, 0,
                        card_width, 0,
                        card_width, card_height,
                        0, card_height,
                        0, top_margin)}
                    fill="#f4e8d0"
                    stroke="#999"
                    stroke-width="2" />

                // Printed characters at top
                {
                    if card.card_type() == CardType::Text {
                        (0..80).filter_map(|col_idx| {
                            if let Some(column) = card.get_column(col_idx)
                                && let Some(ch) = column.printed_char {
                                    let x = left_margin + col_idx as f64 * col_width + col_width / 2.0;
                                    return Some(html! {
                                        <text x={x.to_string()} y={text_y.to_string()}
                                              text-anchor="middle" font-size="10"
                                              font-family="Courier New, monospace" fill="#000">
                                            { ch }
                                        </text>
                                    });
                                }
                            None
                        }).collect::<Html>()
                    } else {
                        html! {}
                    }
                }

                // Column highlight
                {
                    if let Some(col) = current_col {
                        if col < 80 {
                            let x = left_margin + col as f64 * col_width;
                            let highlight_height = card_height - grid_start_y;
                            html! {
                                <rect x={x.to_string()} y={grid_start_y.to_string()}
                                      width={col_width.to_string()}
                                      height={highlight_height.to_string()}
                                      fill="#4a90e2" fill-opacity="0.2" />
                            }
                        } else {
                            html! {}
                        }
                    } else {
                        html! {}
                    }
                }

                // Guide holes
                {
                    (0..80).flat_map(|col_idx| {
                        (0..12).map(move |row_idx| {
                            let x = left_margin + col_idx as f64 * col_width + col_width / 2.0;
                            let y = grid_start_y + row_idx as f64 * row_height + row_height / 2.0;

                            html! {
                                <ellipse cx={x.to_string()}
                                         cy={y.to_string()}
                                         rx={(guide_width / 2.0).to_string()}
                                         ry={(guide_height / 2.0).to_string()}
                                         fill="none"
                                         stroke="#ccc"
                                         stroke-width="0.5" />
                            }
                        })
                    }).collect::<Html>()
                }

                // Pre-printed digits 0-9
                {
                    (0..80).flat_map(|col_idx| {
                        (0..10).map(move |digit| {
                            let x = left_margin + col_idx as f64 * col_width + col_width / 2.0;
                            let row_idx = digit + 2;
                            let y = grid_start_y + row_idx as f64 * row_height + row_height / 2.0 + 3.0;

                            html! {
                                <text x={x.to_string()} y={y.to_string()}
                                      text-anchor="middle" font-size="8" fill="#ccc"
                                      font-family="Courier New, monospace" font-weight="bold">
                                    { digit }
                                </text>
                            }
                        })
                    }).collect::<Html>()
                }

                // Actual punches
                {
                    (0..80).flat_map(|col_idx| {
                        (0..12).filter_map(move |row_idx| {
                            let x = left_margin + col_idx as f64 * col_width + col_width / 2.0;
                            let y = grid_start_y + row_idx as f64 * row_height + row_height / 2.0;

                            if let Some(column) = card.get_column(col_idx) {
                                let punch_array = column.punches.as_array();
                                if punch_array[row_idx] {
                                    return Some(html! {
                                        <rect x={(x - punch_width / 2.0).to_string()}
                                              y={(y - punch_height / 2.0).to_string()}
                                              width={punch_width.to_string()}
                                              height={punch_height.to_string()}
                                              fill="#000" rx="1" />
                                    });
                                }
                            }
                            None
                        })
                    }).collect::<Html>()
                }
            </svg>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_default() {
        let deck = Deck::default();
        assert_eq!(deck.cards.len(), 1);
        assert_eq!(deck.current_card, 0);
    }

    #[test]
    fn test_deck_add_card() {
        let mut deck = Deck::default();
        deck.add_card();
        assert_eq!(deck.cards.len(), 2);
        assert_eq!(deck.current_card, 1);
    }

    #[test]
    fn test_deck_navigation() {
        let mut deck = Deck::default();
        deck.add_card();
        deck.add_card();

        assert_eq!(deck.current_card, 2);
        deck.prev_card();
        assert_eq!(deck.current_card, 1);
        deck.prev_card();
        assert_eq!(deck.current_card, 0);
        deck.prev_card();
        assert_eq!(deck.current_card, 0); // Can't go below 0

        deck.next_card();
        assert_eq!(deck.current_card, 1);
        deck.next_card();
        assert_eq!(deck.current_card, 2);
        deck.next_card();
        assert_eq!(deck.current_card, 2); // Can't go past end
    }

    #[test]
    fn test_deck_delete_card() {
        let mut deck = Deck::default();
        deck.add_card();
        deck.add_card();

        assert_eq!(deck.cards.len(), 3);
        deck.delete_card();
        assert_eq!(deck.cards.len(), 2);
        assert_eq!(deck.current_card, 1);
    }

    #[test]
    fn test_deck_binary_roundtrip() {
        let mut deck = Deck::default();
        let _ = deck.current_mut().set_column_char(0, 'H');
        let _ = deck.current_mut().set_column_char(1, 'I');
        deck.add_card();
        let _ = deck.current_mut().set_column_char(0, 'B');
        let _ = deck.current_mut().set_column_char(1, 'Y');

        let binary = deck.to_binary();
        let loaded = Deck::from_binary(&binary);

        assert_eq!(loaded.cards.len(), 2);
    }
}
