use termcolor::{Color, ColorSpec, StandardStream, WriteColor};
use std::io::{Result, Write};
use packadvice::result::PackResult;
use crate::log::stdout;

fn write_title(stdout: &mut StandardStream, title: &str) -> Result<()> {
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true).set_underline(true))?;
    writeln!(stdout, "{}", title)?;
    stdout.reset()
}

pub fn cli_output(result: &PackResult) -> Result<()> {
    let mut stdout = stdout();
    write_title(&mut stdout, "Pack meta")?;
    writeln!(stdout, "  Pack format: {}", result.pack.pack_meta.pack_format)?;
    writeln!(stdout, "  Minecraft version: {}", result.pack.pack_meta.minecraft_version())?;
    if !result.unreferenced_texture_checker.textures.is_empty() {
        writeln!(stdout)?;
        write_title(&mut stdout, "Unused textures")?;
        for texture in &result.unreferenced_texture_checker.textures {
            writeln!(stdout, " - {}", texture)?;
        }
    }
    if !result.unreferenced_model_checker.models.is_empty() {
        writeln!(stdout)?;
        write_title(&mut stdout, "Unreferenced models")?;
        for model in &result.unreferenced_model_checker.models {
            writeln!(stdout, " - {}", model)?;
        }
    }
    if !result.missing_texture_checker.models.is_empty() {
        writeln!(stdout)?;
        write_title(&mut stdout, "Models that contain #missing texture")?;
        for model in &result.missing_texture_checker.models {
            writeln!(stdout, " - {}", model)?;
        }
    }
    if !result.model_elements_counter.models.is_empty() {
        writeln!(stdout)?;
        write_title(&mut stdout, "List of models and number of elements")?;
        for (model, size) in &result.model_elements_counter.models {
            writeln!(stdout, "  {}: {}", model, size)?;
        }
    }
    stdout.reset()
}
