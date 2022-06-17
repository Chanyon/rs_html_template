use crate::tag::{block, html, link, meta, style, tag, BlockTagConfig, Tag};
use std::collections::HashMap;

pub fn layout() -> Tag {
    let link = link(HashMap::from([(
        "scr".to_string(),
        vec!["./style.css".to_string()],
    )]));
    let title = tag(Tag {
        name: "title".to_string(),
        attr: None,
        children: None,
        text: Some("hello".to_string()),
        empty: Some(false),
    });
    let s = "
    * {
      margin: 0;
      padding: 0;
    }
    body {
      display:flex;
      justify-content:center;
      align-items:center;
      width: 100vw;
      height: 100vh;
      background-image: linear-gradient(to bottom,rgba(0, 0, 255, 0.5), rgba(255, 255, 0, 0.5));
      background-size: cover;
        font-family: 'Franklin Gothic Medium', 'Arial Narrow', Arial, sans-serif;
    
    }
    .form-item{
      display:flex;
      justify-content:space-evenly;
      align-items:center;
    }
    form {
      display:flex;
      flex-direction: column;
      justify-content:space-evenly;
      align-items:center;
      width:350px;
      height:220px;
      padding:40px;
      font-size: 16px;
      text-align:center;
      background:inherit;
      border-radius: 18px;
    }
    form input,form button {
      margin: 6px 0;
      padding: 0 14px;
      height: 35px;
      border: none;
      border-radius: 4px;
      color: #3d5245;
    }
    form button {
      display:block;
      width:240px;
    }
    form button:hover{
      color: #fff;
      background-color:rgba(144,144,237,.7);
    }
";
    let style = style(s.to_string());

    let form = block(BlockTagConfig {
        name: "form".to_string(),
        attr: Some(HashMap::from([(
            "action".to_string(),
            vec!["#".to_string()],
        )])),
        children: Some(vec![
            block(BlockTagConfig {
                name: "div".to_string(),
                attr: Some(HashMap::from([(
                    "class".to_string(),
                    vec!["form-item".to_string()],
                )])),
                children: Some(vec![
                    tag(Tag {
                        name: "label".to_string(),
                        attr: Some(HashMap::from([(
                            "for".to_string(),
                            vec!["name".to_string()],
                        )])),
                        children: None,
                        text: Some("Name".to_string()),
                        empty: Some(false),
                    }),
                    tag(Tag {
                        name: "input".to_string(),
                        attr: Some(HashMap::from([
                            ("type".to_string(), vec!["text".to_string()]),
                            ("id".to_string(), vec!["name".to_string()]),
                        ])),
                        children: None,
                        text: None,
                        empty: Some(true),
                    }),
                ]),
            }),
            block(BlockTagConfig {
                name: "div".to_string(),
                attr: Some(HashMap::from([(
                    "class".to_string(),
                    vec!["form-item".to_string()],
                )])),
                children: Some(vec![
                    tag(Tag {
                        name: "label".to_string(),
                        attr: Some(HashMap::from([(
                            "for".to_string(),
                            vec!["password".to_string()],
                        )])),
                        children: None,
                        text: Some("Password".to_string()),
                        empty: Some(false),
                    }),
                    tag(Tag {
                        name: "input".to_string(),
                        attr: Some(HashMap::from([
                            ("type".to_string(), vec!["text".to_string()]),
                            ("id".to_string(), vec!["password".to_string()]),
                        ])),
                        children: None,
                        text: None,
                        empty: Some(true),
                    }),
                ]),
            }),
            tag(Tag {
                name: "button".to_string(),
                attr: Some(HashMap::from([(
                    "type".to_string(),
                    vec!["button".to_string()],
                )])),
                children: None,
                text: Some("Login".to_string()),
                empty: Some(false),
            }),
        ]),
    });

    let meta1 = meta(HashMap::from([(
        "charset".to_string(),
        vec!["UTF-8".to_string()],
    )]));
    let meta2 = meta(HashMap::from([
        (
            "http-equiv".to_string(),
            vec!["X-UA-Compatible".to_string()],
        ),
        ("content".to_string(), vec!["IE=edge".to_string()]),
    ]));
    let meta3 = meta(HashMap::from([
        ("name".to_string(), vec!["viewport".to_string()]),
        (
            "content".to_string(),
            vec!["width=device-width, initial-scale=1.0".to_string()],
        ),
    ]));

    let html = html(vec![
        block(BlockTagConfig {
            name: "head".to_string(),
            attr: None,
            children: Some(vec![meta1, meta2, meta3, link, title, style]),
        }),
        block(BlockTagConfig {
            name: "body".to_string(),
            attr: None,
            children: Some(vec![form]),
        }),
    ]);
    html
}
