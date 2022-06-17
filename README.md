# rs_html_template
rust spawn html template

## example
```
use rs_html_template::tag::{block, br, html, meta, tag, BlockTagConfig, Tag, link};
use std::collections::HashMap;

fn main() {
    let div = tag(Tag {
        name: "div".to_string(),
        attr: Some(HashMap::from([(
            "class".to_string(),
            vec!["profile".to_string(), "profile1".to_string()],
        )])),
        children: None,
        text: Some("hello".to_string()),
        empty: Some(false),
    });
    let br = br();
    let meta = meta(HashMap::from([
        (
            "name".to_string(),
            vec!["referrer".to_string()],
        ),
        (
            "content".to_string(),
            vec!["origin-when-cross-origin".to_string()],
        ),
    ]));
    let link = link(HashMap::from([
      (
          "scr".to_string(),
          vec!["./style.css".to_string()],
      ),
  ]));
    let html = html(vec![
        block(BlockTagConfig {
            name: "head".to_string(),
            attr: None,
            children: None,
        }),
        block(BlockTagConfig {
            name: "body".to_string(),
            attr: None,
            children: None,
        }),
    ]);
    println!("{}", div.html_template());
    println!("{}", br.html_template());
    println!("{}", meta.html_template());
    println!("{}", link.html_template());
    println!("{}", html.html_template());
}

// cargo run > index.html
```