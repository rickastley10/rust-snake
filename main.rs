use ratatui::{
    crossterm::event::{self, Event, KeyCode}, // Added Event
    layout::Position,
    style::{Color, Style, Stylize},
    DefaultTerminal, Frame,
};
use std::time::{Duration, Instant};
use rand::Rng;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    // Explicitly annotate as u16
    let mut head_x: u16 = 10;
    let mut head_y: u16 = 10;
    let mut seg_x: Vec<u16> = vec![10, 9, 8];
    let mut seg_y: Vec<u16> = vec![10, 10, 10];

    let mut pointer = 0;
    let mut segments = 3;
    let mut dir = 2;

    let tick_rate = Duration::from_millis(150);
    let mut last_tick = Instant::now();

    let mut rng = rand::thread_rng();
    let mut berry_x: u16 = rng.gen_range(1..20);
    let mut berry_y: u16 = rng.gen_range(1..20);

    loop {
        if event::poll(Duration::from_millis(10))? {
            // FIX: Pattern match on Event::Key instead of using as_key_press_event()
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
                if key.code == KeyCode::Char('w') { dir = 1; }
                if key.code == KeyCode::Char('d') { dir = 2; }
                if key.code == KeyCode::Char('s') { dir = 3; }
                if key.code == KeyCode::Char('a') { dir = 4; }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
            if dir == 1 { head_y = head_y.saturating_sub(1); }
            if dir == 2 { head_x = head_x.saturating_add(1); }
            if dir == 3 { head_y = head_y.saturating_add(1); }
            if dir == 4 { head_x = head_x.saturating_sub(1); }

            for i in 0..segments {
                if head_x == seg_x[i] && head_y == seg_y[i] {
                    return Ok(());
                }
            }
            if head_x == berry_x && head_y == berry_y {
                seg_x.push(head_x);
                seg_y.push(head_y);

                // FIX: Randomize berry_x, not berry_y twice
                berry_x = rng.gen_range(1..20);
                berry_y = rng.gen_range(1..20);
            }

            seg_x[pointer] = head_x;
            seg_y[pointer] = head_y;

            pointer = pointer.saturating_add(1);
            if pointer >= segments {
                pointer = 0;
            }
        }

        terminal.draw(|frame| {
            draw_pixel(frame, berry_x * 2, berry_y, Color::Red);
            draw_pixel(frame, berry_x * 2 - 1, berry_y, Color::Red);

            draw_pixel(frame, head_x * 2 - 1, head_y, Color::Gray);
            draw_pixel(frame, head_x * 2 - 1, head_y, Color::Gray);
            for i in 0..segments {
                draw_pixel(frame, seg_x[i] * 2, seg_y[i], Color::Yellow);
                draw_pixel(frame, seg_x[i] * 2 - 1, seg_y[i], Color::Yellow);
            }
        })?;
    }
}

fn draw_pixel(frame: &mut Frame, x: u16, y: u16, color: Color) {
    if let Some(cell) = frame.buffer_mut().cell_mut(Position::new(x, y)) {
        cell.set_char('█').set_fg(color);
    }
}

fn draw_text(frame: &mut Frame, x: u16, y: u16, text: &str, style: Style) {
    let area = frame.area();
    if y >= area.bottom() || x >= area.right() { return; }
    let max_width = area.right() - x;
    frame.buffer_mut().set_stringn(x, y, text, max_width as usize, style);
}