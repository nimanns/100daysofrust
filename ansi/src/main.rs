use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
    writeln!(&mut stdout, "This text is red.")?;

    stdout.reset()?;
    stdout.set_color(ColorSpec::new().set_bg(Some(Color::Blue)))?;
    writeln!(&mut stdout, "This text has a blue background.")?;

    stdout.reset()?;

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))?;
    writeln!(&mut stdout, "This text is green and bold.")?;

    stdout.reset()?;
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)).set_italic(true))?;
    writeln!(&mut stdout, "This text is magenta and italic.")?;

    stdout.reset()?;

    Ok(())
}

