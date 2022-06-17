use std::collections::HashMap;
pub trait Template {
  // return the tag to encode JSON
  fn temp_str(&self) -> String;
}

#[derive(Clone,Debug)]
pub struct Tag {
  pub name: String,
  pub attr: Option<HashMap<String, Vec<String>>>,
  pub children: Option<Vec<Tag>>,
  pub text: Option<String>,
  pub empty: Option<bool>,
}

pub struct BlockTagConfig {
  pub name: String,
  pub attr: Option<HashMap<String, Vec<String>>>,
  pub children: Option<Vec<Tag>>,
}

impl Template for Tag {
  fn temp_str(&self) -> String {
    let left = "{";
    let right = "}";
    format!("{}name: {},children: {}{}",left,self.name,self.children.as_ref().unwrap().len(),right)
  }
}

impl Tag {
  pub fn is_text(&self) -> bool {
    return 0 == self.name.len() && self.text.as_ref().unwrap().len() > 0;
  }
  
  pub fn html_template(&self) -> String {
    let mut sb = String::new();
    let is_text = self.is_text();
    if !is_text {
      if self.name == "html".to_string() {
        sb.push_str("<!DOCTYPE html>");
      }
      sb.push_str(&format!("<{}",self.name));
      if let Some(attr) = self.attr.as_ref() {
        for (prop_name,prop_value) in attr {
          let name = prop_name;
          let value = prop_value.join(" ");
          sb.push_str(&format!(" {}=\"{}\"",name,value));
        }
      }
      if !self.empty.unwrap() {
        sb.push_str(">")
      }else {
        sb.push_str("/>")
      }
    }
    if let Some(text) = self.text.as_ref() {
      sb.push_str(text);
    }
    if !self.empty.unwrap() {
      if let Some(children) = self.children.as_ref() {
        for child in children {
          sb.push_str(child.html_template().as_str())
        }
      }
      if !is_text {
        sb.push_str(format!("</{}>",self.name).as_str())
      }
    }
    sb
  }
}

pub fn html(children: Vec<Tag>) -> Tag {
  Tag {
    name: String::from("html"),
    attr: None,
    children: Some(children),
    text: None,
    empty: Some(false),
  }
}
pub fn block(tag: BlockTagConfig) -> Tag {
  Tag {
    name: tag.name,
    attr: tag.attr,
    children: tag.children,
    text: None,
    empty: Some(false),
  }
}
pub fn tag(tag: Tag) -> Tag {
  Tag {
    ..tag
  }
}
pub fn meta(attr: HashMap<String, Vec<String>>) -> Tag {
  Tag {
    name: "meta".to_string(),
    attr: Some(attr),
    empty: Some(true),
    text: None,
    children: None,
  }
}
pub fn link(attr: HashMap<String, Vec<String>>) -> Tag {
  Tag {
    name: "link".to_string(),
    attr: Some(attr),
    empty: Some(true),
    text: None,
    children: None,
  }
}
pub fn style(style: String) -> Tag {
  Tag {
    name: "style".to_string(),
    text: Some(style),
    empty: Some(false),
    children: None,
    attr: None,
  }
}
pub fn br() -> Tag {
  Tag {
    name: "br".to_string(),
    attr: None,
    children: None,
    text: None,
    empty: Some(true),
  }
}