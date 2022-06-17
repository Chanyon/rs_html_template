use rs_html_template::layout::layout;
fn main() {
    let html = layout();
    println!("{}", html.html_template());
}
