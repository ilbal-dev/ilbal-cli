use owo_colors::OwoColorize;

pub fn gradient(t: f32) -> (u8, u8, u8) {
    let lerp = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * t).round() as u8;
    (lerp(255, 0), lerp(165, 255), lerp(0, 255))
}

pub fn gradient_text(text: &str) -> String {
    let width = text.lines().next().map_or(1, |l| l.chars().count().max(1));
    text.lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .map(|(i, c)| {
                    let t = if width > 1 {
                        i as f32 / (width - 1) as f32
                    } else {
                        0.0
                    };
                    let (r, g, b) = gradient(t);
                    format!("{}", c.truecolor(r, g, b))
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}
