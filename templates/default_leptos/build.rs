fn main() {
    std::fs::write("assets/mrvillage-ui.css", mrvillage_ui::CSS).unwrap();
    std::fs::write("style/mrvillage-ui.css", mrvillage_ui::CSS).unwrap();
}
