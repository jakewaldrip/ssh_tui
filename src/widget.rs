use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::text::Text;
use ratatui::widgets::Block;
use ratatui::{backend::CrosstermBackend, widgets::Clear, Terminal};

use crate::app::App;
use crate::terminal_handle::TerminalHandle;

/* TODO
* [X] Setup Kris Facts Basic Overall
* [X] Create menu layout
* [X] Create Footer Layout
* [X] Create Content Box for Facts
* [X] Create Title layout
* [ ] Implement Menu Selection Visuals
* [ ] Switch to new placeholder Content/Footer on new page
* [ ] Pull initial fact from static arr
* [ ] Change fact content when key is pressed
* [ ] Implement Content/Footer for About Page
* [ ] Refactor/Cleanup Widget Code
*/
pub fn render_ui(terminal: &mut Terminal<CrosstermBackend<TerminalHandle>>, app: &mut App) -> () {
    terminal
        .draw(|frame| {
            let frame_area = frame.area();
            frame.render_widget(Clear, frame_area);

            let [main_container] = Layout::horizontal([Constraint::Percentage(50)])
                .flex(Flex::Center)
                .areas(frame_area);

            let [padding_row_top, title_row, menu_row, content_row, footer_row, padding_row_bottom] = Layout::vertical([
                Constraint::Fill(1),
                Constraint::Percentage(15),
                Constraint::Length(3),
                Constraint::Percentage(35),
                Constraint::Length(3),
                Constraint::Fill(1),
            ])
            .areas(main_container);

            // Title Row
            let title_text = Text::raw("Kris Facts");
            let title_area = center(
                title_row,
                Constraint::Length(title_text.width() as u16),
                Constraint::Length(1),
            );

            // Menu Row
            let menu_area = center(menu_row, Constraint::Fill(1), Constraint::Fill(1));
            let [left_menu_area, right_menu_area] =
                Layout::horizontal([Constraint::Percentage(50); 2]).areas(menu_area);

            let facts_menu_text = Text::raw("[f]acts");
            let facts_menu_area = center(
                left_menu_area,
                Constraint::Length(facts_menu_text.width() as u16),
                Constraint::Length(1),
            );
            let facts_menu_block = Block::bordered();

            let about_menu_text = Text::raw("[a]bout");
            let about_menu_area = center(
                right_menu_area,
                Constraint::Length(about_menu_text.width() as u16),
                Constraint::Length(1),
            );
            let about_menu_block = Block::bordered();

            // Footer Row
            let footer_area = center(footer_row, Constraint::Percentage(33), Constraint::Fill(1));
            let [left_footer_area, right_footer_area] =
                Layout::horizontal([Constraint::Percentage(50); 2]).areas(footer_area);

            let new_fact_footer_text = Text::raw("[n]ew fact");
            let new_fact_footer_area = center(
                left_footer_area,
                Constraint::Length(new_fact_footer_text.width() as u16),
                Constraint::Length(1),
            );

            let quit_footer_text = Text::raw("[q]uit");
            let quit_footer_area = center(
                right_footer_area,
                Constraint::Length(quit_footer_text.width() as u16),
                Constraint::Length(1),
            );

            // Content Row
            let content = app.fact.clone();
            let content_text = Text::raw(&*content);
            let content_area = center(
                content_row,
                Constraint::Length(content_text.width() as u16),
                Constraint::Length(3),
            );

            // Render widgets
            frame.render_widget(Block::new(), padding_row_top);
            frame.render_widget(Block::new(), padding_row_bottom);

            frame.render_widget(title_text, title_area);

            frame.render_widget(facts_menu_text, facts_menu_area);
            frame.render_widget(facts_menu_block, left_menu_area);
            frame.render_widget(about_menu_text, about_menu_area);
            frame.render_widget(about_menu_block, right_menu_area);

            frame.render_widget(Block::bordered(), content_row);
            frame.render_widget(content_text, content_area);

            frame.render_widget(new_fact_footer_text, new_fact_footer_area);
            frame.render_widget(quit_footer_text, quit_footer_area);
        })
        .unwrap();
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
