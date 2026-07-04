use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Frame,
};

use crate::system::SystemStats;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Component {
    Cpu,
    Gpu,
    UnifiedMemory,
    NeuralEngine,
    SsdController,
    SecureEnclave,
}

impl Component {
    pub fn name(&self) -> &'static str {
        match self {
            Component::Cpu => "Central Processing Unit (CPU)",
            Component::Gpu => "Graphics Processing Unit (GPU)",
            Component::UnifiedMemory => "Unified Memory (RAM)",
            Component::NeuralEngine => "Apple Neural Engine (ANE)",
            Component::SsdController => "SSD & Storage Controller",
            Component::SecureEnclave => "Secure Enclave (Security Chip)",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Component::Cpu => Component::Gpu,
            Component::Gpu => Component::UnifiedMemory,
            Component::UnifiedMemory => Component::NeuralEngine,
            Component::NeuralEngine => Component::SsdController,
            Component::SsdController => Component::SecureEnclave,
            Component::SecureEnclave => Component::Cpu,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Component::Cpu => Component::SecureEnclave,
            Component::Gpu => Component::Cpu,
            Component::UnifiedMemory => Component::Gpu,
            Component::NeuralEngine => Component::UnifiedMemory,
            Component::SsdController => Component::NeuralEngine,
            Component::SecureEnclave => Component::SsdController,
        }
    }
}

pub fn draw(f: &mut Frame, stats: &SystemStats, active: Component) {
    let size = f.size();

    // Check if terminal is too small
    if size.width < 80 || size.height < 24 {
        let warning = Paragraph::new("Terminal too small! Please resize to at least 80x24.")
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center);
        f.render_widget(warning, size);
        return;
    }

    // Main Layout: Header, Content, Footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3), // Header
                Constraint::Min(10),   // Content Area
                Constraint::Length(1), // Footer/Keys
            ]
            .as_ref(),
        )
        .split(size);

    // 1. Render Header
    let header = Paragraph::new(vec![Line::from(vec![
        Span::styled(" 🍏 MacAnatomy TUI ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(" - Interactive Apple Silicon Hardware Explorer", Style::default().fg(Color::Gray)),
    ])])
    .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)))
    .alignment(Alignment::Left);
    f.render_widget(header, chunks[0]);

    // 2. Split Content Area: Left (SoC Visual Diagram), Right (Stats & Lexicon)
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)].as_ref())
        .split(chunks[1]);

    draw_soc_diagram(f, content_chunks[0], active, stats);
    draw_details_panel(f, content_chunks[1], active, stats);

    // 3. Render Footer
    let footer_text = vec![Line::from(vec![
        Span::styled(" ←/→/↑/↓ or Tab ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled("Navigate Components  |  ", Style::default().fg(Color::Gray)),
        Span::styled(" Q ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        Span::styled("Quit", Style::default().fg(Color::Gray)),
    ])];
    let footer = Paragraph::new(footer_text).alignment(Alignment::Center);
    f.render_widget(footer, chunks[2]);
}

fn draw_soc_diagram(f: &mut Frame, area: Rect, active: Component, stats: &SystemStats) {
    let soc_block = Block::default()
        .title(" System-on-Chip (SoC) Package Map ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray));

    let inner_area = soc_block.inner(area);
    f.render_widget(soc_block, area);

    // Subdivide the SoC inner area into layout blocks
    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)].as_ref())
        .split(inner_area);

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(v_chunks[0]);

    let top_left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(top_chunks[0]);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(v_chunks[1]);

    let bottom_right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(bottom_chunks[1]);

    let render_comp_block = |f: &mut Frame, dest: Rect, comp: Component, title: &str, subtitle: &str| {
        let is_active = active == comp;
        let border_color = if is_active { Color::Green } else { Color::DarkGray };
        let title_style = if is_active {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let block = Block::default()
            .title(Span::styled(format!(" {} ", title), title_style))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color));

        let text = vec![
            Line::from(""),
            Line::from(Span::styled(subtitle, Style::default().fg(Color::Gray))),
        ];
        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center);

        f.render_widget(paragraph, dest);
    };

    // Render each CPU/GPU/RAM block
    render_comp_block(
        f,
        top_left_chunks[0],
        Component::Cpu,
        "CPU Cores",
        &format!("{} Cores", stats.cpu_cores),
    );
    render_comp_block(
        f,
        top_left_chunks[1],
        Component::Gpu,
        "GPU Cores",
        "Integrated Graphics",
    );
    
    // RAM block spans more vertical space
    let ram_gb = (stats.total_memory as f64 / 1024.0 / 1024.0 / 1024.0).round();
    let ram_sub = format!("{:.0} GB Unified", ram_gb);
    render_comp_block(
        f,
        top_chunks[1],
        Component::UnifiedMemory,
        "Unified Memory",
        &ram_sub,
    );

    render_comp_block(
        f,
        bottom_chunks[0],
        Component::NeuralEngine,
        "Neural Engine",
        "AI Cores",
    );
    render_comp_block(
        f,
        bottom_right_chunks[0],
        Component::SsdController,
        "SSD Controller",
        "Storage Link",
    );
    render_comp_block(
        f,
        bottom_right_chunks[1],
        Component::SecureEnclave,
        "Secure Enclave",
        "Hardware Vault",
    );
}

fn draw_details_panel(f: &mut Frame, area: Rect, active: Component, stats: &SystemStats) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(8)].as_ref())
        .split(area);

    // 1. Render Top Details: Live system metrics
    let stats_block = Block::default()
        .title(" Host System Live Metrics ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray));

    let stats_inner = stats_block.inner(chunks[0]);
    f.render_widget(stats_block, chunks[0]);

    let stats_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(2), Constraint::Length(2)].as_ref())
        .split(stats_inner);

    // Host / OS line
    let host_text = Paragraph::new(vec![Line::from(vec![
        Span::styled("Host: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{} ", stats.host_name), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled(" | OS: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{} {} ", stats.os_name, stats.os_version), Style::default().fg(Color::White)),
        Span::styled(" | CPU: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{} ", stats.cpu_name), Style::default().fg(Color::Cyan)),
    ])]);
    f.render_widget(host_text, stats_layout[0]);

    // Live meters
    let meters_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(stats_layout[1]);

    // CPU Usage
    let cpu_gauge = Gauge::default()
        .block(Block::default().title("Overall CPU Usage"))
        .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
        .percent(stats.overall_cpu_usage as u16);
    f.render_widget(cpu_gauge, meters_layout[0]);

    // Memory Usage
    let mem_used_gb = stats.used_memory as f64 / 1024.0 / 1024.0 / 1024.0;
    let mem_total_gb = stats.total_memory as f64 / 1024.0 / 1024.0 / 1024.0;
    let mem_gauge = Gauge::default()
        .block(Block::default().title(format!("Memory: {:.1}G / {:.1}G", mem_used_gb, mem_total_gb)))
        .gauge_style(Style::default().fg(Color::Magenta).bg(Color::DarkGray))
        .percent(stats.memory_percentage as u16);
    f.render_widget(mem_gauge, meters_layout[1]);

    // SSD storage usage line
    let ssd_used_gb = stats.used_disk as f64 / 1024.0 / 1024.0 / 1024.0;
    let ssd_total_gb = stats.total_disk as f64 / 1024.0 / 1024.0 / 1024.0;
    let ssd_gauge = Gauge::default()
        .block(Block::default().title(format!("SSD Space: {:.0}G / {:.0}G", ssd_used_gb, ssd_total_gb)))
        .gauge_style(Style::default().fg(Color::Yellow).bg(Color::DarkGray))
        .percent(stats.disk_percentage as u16);
    f.render_widget(ssd_gauge, stats_layout[2]);

    // 2. Render Bottom Details: Hardware Lexicon
    let lexicon_block = Block::default()
        .title(format!(" Anatomy Lexicon: {} ", active.name()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let (analogy, description, fail_symptom) = get_explanation(active, stats);

    let mut lexicon_text = vec![
        Line::from(vec![
            Span::styled("💡 Analogy: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(analogy, Style::default().fg(Color::White).add_modifier(Modifier::ITALIC)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("📖 How it works: ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]),
    ];

    // Wrap description paragraphs nicely
    for line in description.split('\n') {
        lexicon_text.push(Line::from(Span::styled(line, Style::default().fg(Color::Gray))));
    }

    lexicon_text.push(Line::from(""));
    lexicon_text.push(Line::from(vec![
        Span::styled("⚠️ Failure / Symptoms: ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        Span::styled(fail_symptom, Style::default().fg(Color::White)),
    ]));

    let lexicon_p = Paragraph::new(lexicon_text)
        .block(lexicon_block)
        .wrap(Wrap { trim: true });

    f.render_widget(lexicon_p, chunks[1]);
}

fn get_explanation(comp: Component, stats: &SystemStats) -> (&'static str, String, &'static str) {
    match comp {
        Component::Cpu => (
            "The head chef in a busy kitchen.",
            format!(
                "The CPU (Central Processing Unit) handles all primary commands and logic calculations. \n\
                On your Mac ({}), this chip contains {} processor cores. \n\
                Apple Silicon utilizes a hybrid design combining high-performance cores (for rendering video, games, compiling code) and high-efficiency cores (for low-power background jobs like checking emails), optimizing battery life.",
                stats.cpu_name, stats.cpu_cores
            ),
            "If the CPU fails or overheats (thermal throttling), apps will freeze, become sluggish, or the Mac will instantly shut down to prevent permanent heat damage.",
        ),
        Component::Gpu => (
            "The speed-painter who draws everything you see.",
            "The GPU (Graphics Processing Unit) specializes in math for visual tasks: rendering the macOS desktop, playing video, and games. \n\
            In your Mac, the GPU is integrated directly into the M-series system chip. It has direct access to the same Unified Memory as the CPU, meaning it can fetch textures and graphics assets near-instantly without lagging data transfers.".to_string(),
            "If the GPU malfunctions, you will see visual glitches (artifacts), flickering screens, or screen freezes when playing video or resizing windows.",
        ),
        Component::UnifiedMemory => (
            "The chef's kitchen countertop.",
            format!(
                "Unified Memory acts as the high-speed workspace. Your Mac has {:.0} GB of it. \n\
                Unlike a regular PC where RAM is located far away on slot sticks, Apple Silicon integrates memory chips directly inside the processor package. \n\
                Because the CPU, GPU, and Neural Engine share this single pool of memory, they don't need to copy data back and forth. This means immediate performance with zero redundant memory overhead.",
                stats.total_memory as f64 / 1024.0 / 1024.0 / 1024.0
            ),
            "If you run out of memory, macOS will use virtual memory ('swap') to write files to your SSD, slowing things down. If RAM chips physically break, the Mac won't turn on and will beep repeatedly.",
        ),
        Component::NeuralEngine => (
            "The translation specialist or calculator whiz.",
            "The Neural Engine is a dedicated coprocessor optimized specifically for machine learning and AI math. \n\
            It handles tasks like Voice Dictation, face detection in photos, handwriting recognition, and predicting text. It can process trillions of operations per second while drawing almost zero power, keeping your CPU and GPU free.".to_string(),
            "If disabled, AI features like live captions, audio transcription, or photo searches will run much slower because they must fallback to using the CPU cores.",
        ),
        Component::SsdController => (
            "The library filing cabinet and fast archivist.",
            format!(
                "The SSD (Solid State Drive) is your long-term storage, keeping all your files, OS, and apps. \n\
                Your Mac is currently using {:.1} GB out of {:.1} GB. \n\
                The controller is integrated on the Apple Silicon chip, interfacing directly with the storage chips. This custom controller enables read and write speeds of several gigabytes per second, enabling your Mac to boot and open apps almost instantly.",
                stats.used_disk as f64 / 1024.0 / 1024.0 / 1024.0,
                stats.total_disk as f64 / 1024.0 / 1024.0 / 1024.0
            ),
            "If the SSD fails or runs completely full, your Mac might boot loop, refuse to open files, crash during writes, or show a blinking folder icon with a question mark at startup.",
        ),
        Component::SecureEnclave => (
            "A high-security bank vault with its own guard.",
            "The Secure Enclave is a separate security coprocessor isolated from the main CPU. \n\
            It manages its own encrypted storage and handles your Touch ID verification, passcode logic, Apple Pay keys, and file encryption keys (FileVault). \n\
            Even if macOS is fully compromised by malware, the malware cannot access the keys or biomatric data inside the Secure Enclave.".to_string(),
            "If compromised or failing, Touch ID will stop working, passwords cannot be saved or checked safely, and Apple Pay will be disabled.",
        ),
    }
}
