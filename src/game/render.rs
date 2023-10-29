use super::*;

impl Game {
    pub fn render(&self) -> Paragraph<'_> {
        Paragraph::new("game board")
            .block(
                Block::default()
                    .borders(Borders::all())
                    .border_type(BorderType::Rounded),
            )
            .alignment(Alignment::Center)
    }
}
