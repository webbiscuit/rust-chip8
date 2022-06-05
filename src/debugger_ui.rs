use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Paragraph, List, ListItem};
use tui::Frame;

use crate::cpu::Cpu;
// use unicode_width::UnicodeWidthStr;

// use crate::app::App;

pub fn draw<B>(rect: &mut Frame<B>, cpu: &Cpu)
where
    B: Backend,
{
    let size = rect.size();
    // check_size(&size);

    // Vertical layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(12)].as_ref())
        .split(size);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(23), Constraint::Min(12)].as_ref())
        .split(chunks[1]);

    // Add widgets
    let title = draw_title();
    let cpu_ui = draw_cpu(cpu);
    // let user_input = draw_user_input(app);
    rect.render_widget(title, chunks[0]);
    // rect.render_widget(main_chunks, chunks[1]);
    rect.render_widget(cpu_ui, main_chunks[0]);
    // rect.render_widget(user_input, main_chunks[1]);

    // rect.set_cursor(
    //     main_chunks[1].x + app.ui_buffer.width() as u16 + 1,
    //     main_chunks[1].y + 2,
    // )
}

fn draw_cpu(cpu: &Cpu) -> List {
    let mut items: Vec<ListItem> = Vec::new();

    items.push(ListItem::new(format!(" {:>3} - {}", "Cycle", cpu.cycle_count())));
    items.push(ListItem::new(format!(" {:>3} - {:#06X}", "PC", cpu.program_counter())));
    items.push(ListItem::new(format!(" {:>3} - {:#06X}", "I", cpu.i_register())));
    cpu.v_registers().iter().enumerate().for_each(|(i, v)| {
        let v_format = format!("V{}", i);
        items.push(ListItem::new(format!(" {:>3} - {:#04X}", v_format, v)));
    });

    // items.push(Spans::from(Span::styled(
    //     "CPU",
    //     Style::default().fg(Color::Yellow),
    // )));

    // items.push(Spans::from(Span::styled(
    //     format!("{:04X}", 0xabcd),
    //     Style::default().fg(Color::Yellow),
    // )));

    List::new(items)
        .block(
            Block::default()
                .title("CPU")
                .borders(Borders::ALL),
        )
        // .highlight_symbol(">")
        // .highlight_style(Style::default().fg(Color::Yellow))
        
        // .highlight_style(Style::default().modifier(StyleModifier::BOLD))
        // .highlight_symbol_style(Style::default().fg(Color::Yellow))
        // .highlight_symbol_style(Style::default().modifier(StyleModifier::BOLD))
        // .render(0, 0)
        
}



    // let mut items = Vec::new();
    // items.push(Spans::from(Span::styled(
    //     "PC: ".to_string(),
    //     Style::default().fg(Color::Yellow),
    // )));
    // items.push(Spans::from(Span::styled(
    //     format!("{:04X}", 0xabcd),
    //     Style::default().fg(Color::White),
    // )));
    // // items.push(Spans::from(Span::styled(
    // //     "SP: ".to_string(),
    // //     Style::default().fg(Color::Yellow),
    // // )));
    // // items.push(Spans::from(Span::styled(
    // //     format!("{:02X}", cpu.sp),
    // //     Style::default().fg(Color::White),
    // // )));
    // // items.push(Spans::from(Span::styled(
    // //     "I: ".to_string(),
    // //     Style::default().fg(Color::Yellow),
    // // )));
    // // items.push(Spans::from(Span::styled(
    // //     format!("{:02X}", cpu.i),
    // //     Style::default().fg(Color::White),
    // // )));
    // // items.push(Spans::from(Span::styled(
    // //     "DT: ".to_string(),
    // //     Style::default().fg(Color::Yellow),
    // // )));
    // // items.push(Spans::from(Span::styled(
    // //     format!("{:02X}", cpu.dt),
    // //     Style::default().fg(Color::White),
    // // )));
    // // items.push(Spans::from(Span::styled(
    // //     "ST: ".to_string(),
    // //     Style::default().fg(Color::Yellow),
    // // )));
    // // items.push(Spans::from(Span::styled(
    // //     format!("{:02X}", cpu.st),
    // //     Style::default().fg(Color::White),
    // // )));
    // // items.push(Spans::from(Span::styled(
    // //     "V[0]: ".to_string(),
    // //     Style::default().fg(Color::Yellow),
    // // )));
    // // items

    // List::new(items)
    //     .block(
    //         Block::default()
    //             .borders(Borders::ALL)
    //             .title("CPU"),
    //     )
    //     .style(Style::default().fg(Color::White))
    //     .highlight_style(Style::default().fg(Color::Yellow))
    //     .highlight_symbol(">>")
        
//}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Dan's Rusty Chip8 ⌨")
        .style(Style::default().fg(Color::LightMagenta))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}

