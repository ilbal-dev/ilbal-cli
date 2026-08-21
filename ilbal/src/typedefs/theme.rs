use console::style;
use dialoguer::theme::Theme;
use std::fmt;

pub struct SelectTheme;

impl Theme for SelectTheme {
    fn format_select_prompt_item(
        &self,
        f: &mut dyn fmt::Write,
        text: &str,
        active: bool,
    ) -> fmt::Result {
        if active {
            write!(
                f,
                "{} {}",
                style(">").for_stderr().cyan().bold(),
                style(text).for_stderr().cyan().bold()
            )
        } else {
            write!(f, "  {text}")
        }
    }
}
