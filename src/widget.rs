use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::text::Text;
use ratatui::widgets::Block;
use ratatui::{backend::CrosstermBackend, widgets::Clear, Terminal};

use crate::app::{App, MenuSelection};
use crate::terminal_handle::TerminalHandle;

/* TODO
* [X] Setup Kris Facts Basic Overall
* [X] Create menu layout
* [X] Create Footer Layout
* [X] Create Content Box for Facts
* [X] Create Title layout
* [X] Implement Menu Selection Visuals
* [X] Switch to new placeholder Content/Footer on new page
* [X] Pull initial fact from static arr
* [X] Change fact content when key is pressed
* [X] Implement Content/Footer for About Page
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

            let facts_menu_text = match &app.menu_selection {
                MenuSelection::Facts => Text::raw("[f]acts").white(),
                MenuSelection::About => Text::raw("[f]acts").dark_gray(),
            };
            let facts_menu_block = match &app.menu_selection {
                MenuSelection::Facts => Block::bordered().white(),
                MenuSelection::About => Block::bordered().dark_gray(),
            };

            let facts_menu_area = center(
                left_menu_area,
                Constraint::Length(facts_menu_text.width() as u16),
                Constraint::Length(1),
            );

            let about_menu_text = match &app.menu_selection {
                MenuSelection::Facts => Text::raw("[a]bout").gray(),
                MenuSelection::About => Text::raw("[a]bout").white(),
            };
            let about_menu_block= match &app.menu_selection {
                MenuSelection::Facts => Block::bordered().gray(),
                MenuSelection::About => Block::bordered().white(),
            };

            let about_menu_area = center(
                right_menu_area,
                Constraint::Length(about_menu_text.width() as u16),
                Constraint::Length(1),
            );

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

            let quit_footer_solo_area = center(
                footer_area,
                Constraint::Length(quit_footer_text.width() as u16),
                Constraint::Length(1),
            );

            // Fact Content Row
            let fact_content = app.fact.clone();
            let fact_content_text = Text::raw(&*fact_content);
            let fact_content_area = center(
                content_row,
                Constraint::Length(fact_content_text.width() as u16),
                Constraint::Length(3),
            );

            // About Content Row
            let about_container_area = center(
                content_row,
                Constraint::Percentage(75),
                Constraint::Percentage(75)
            );

            let [left_about_content_area, right_about_content_area] = Layout::horizontal([Constraint::Fill(1); 2])
                .areas(about_container_area);

            let [left_about_content_row1_area, left_about_content_row2_area, left_about_content_row3_area] = Layout::vertical([Constraint::Fill(1); 3])
                .areas(left_about_content_area);

            let [right_about_content_row1_area, right_about_content_row2_area, right_about_content_row3_area] = Layout::vertical([Constraint::Fill(1); 3])
                .areas(right_about_content_area);

            let left_about_content_row1_text = Text::raw("whoami");
            let left_about_content_row2_text = Text::raw(r#"grep -r "resume.pdf""#);
            let left_about_content_row3_text = Text::raw("./github.sh");

            let right_about_content_row1_text = Text::raw("jacob waldrip");
            let right_about_content_row2_text = Text::raw("https://jacobwaldrip.com");
            let right_about_content_row3_text = Text::raw("https://github.com/jakewaldrip");

            // Render widgets
            frame.render_widget(Block::new(), padding_row_top);
            frame.render_widget(Block::new(), padding_row_bottom);

            frame.render_widget(title_text, title_area);

            frame.render_widget(facts_menu_text, facts_menu_area);
            frame.render_widget(facts_menu_block, left_menu_area);
            frame.render_widget(about_menu_text, about_menu_area);
            frame.render_widget(about_menu_block, right_menu_area);

            frame.render_widget(Block::bordered(), content_row);

            match &app.menu_selection {
                MenuSelection::Facts => {
                    frame.render_widget(fact_content_text, fact_content_area);

                    frame.render_widget(new_fact_footer_text, new_fact_footer_area);
                    frame.render_widget(quit_footer_text, quit_footer_area);
                },
                MenuSelection::About => {
                    frame.render_widget(left_about_content_row1_text, left_about_content_row1_area);
                    frame.render_widget(left_about_content_row2_text, left_about_content_row2_area);
                    frame.render_widget(left_about_content_row3_text, left_about_content_row3_area);
                    frame.render_widget(right_about_content_row1_text, right_about_content_row1_area);
                    frame.render_widget(right_about_content_row2_text, right_about_content_row2_area);
                    frame.render_widget(right_about_content_row3_text, right_about_content_row3_area);

                    frame.render_widget(quit_footer_text, quit_footer_solo_area);
                },
            };

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
