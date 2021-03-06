extern crate failure;
#[macro_use]
extern crate output;
#[macro_use]
extern crate serde_derive;

use output::{
    components::{newline, text},
    human, json,
};

fn main() -> Result<(), failure::Error> {
    let mut out = output::new()
        .add_target(json::file("target/foo.log")?)
        .add_target(human::stdout()?);

    #[derive(Serialize)]
    struct ErrorMessage {
        code: i32,
        name: String,
        message: String,
    }

    impl output::Render for ErrorMessage {
        render_for_humans!(self -> [
            span!(fg = "white", bg = "black", [text(self.code.to_string()), text(" "),]),
            span!(fg = "red", bg = "black", [text(&self.name),]),
            newline(),
            text("> "),
            text(&self.message),
        ]);

        render_json!();
    }

    out.print(&ErrorMessage {
        code: 42,
        name: String::from("error"),
        message: String::from("Oh god no"),
    })?;

    Ok(())
}
