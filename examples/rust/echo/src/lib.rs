wit_bindgen_rust::export!("echo.wit");
struct Echo;

#[debugger_macro::export_debug_handler]
impl echo::Echo for Echo {
    fn echo(phrase: String) -> String {
        dbg!("hello from wasm!", &phrase);
        format!("{} {}", phrase, phrase)
    }
}
