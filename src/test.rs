use pithago_rs::*;
fn main() {
    create_context();
}
fn create_context() {
    let rcb = RenderContext::new();
    rcb.set_title("name");
}
