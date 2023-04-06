use std::{borrow::Cow, time::Instant};

use tl::{NodeHandle, ParseError, Parser, ParserOptions};

pub fn parse_html(html: &str) -> Result<(), ParseError> {
  let now = Instant::now();
  let dom = tl::parse(html, ParserOptions::default())?;
  let parser = dom.parser();
  let table = dom
    .query_selector("table")
    .expect("Unable to perform selection")
    .last()
    .unwrap();

  parse_table(table.get(parser).unwrap().inner_html(parser))?;
  let elapsed = now.elapsed();
  println!("Elapsed: {elapsed:?}");
  Ok(())
}

fn parse_table(html: Cow<str>) -> Result<(), ParseError> {
  let dom = tl::parse(&html, ParserOptions::default())?;
  let parser = dom.parser();
  let trs = dom.query_selector("tr").expect("Unable to select tr");

  let mut table = trs
    .map(|tr| tr.get(parser).unwrap())
    .skip(2)
    .map(|tr| {
      tr.children()
        .expect("Unable to get children")
        .top()
        .iter()
        .filter_map(|handle| get_inner_text(parser, handle))
        .map(normalize)
        .collect::<Vec<String>>()
    })
    .collect::<Vec<Vec<String>>>();

  println!("{:#?}", table);
  Ok(())
}

fn get_inner_text(parser: &Parser, node: &NodeHandle) -> Option<String> {
  let res = node.get(parser)?.inner_text(parser);
  let res = res.trim();
  match res.len() {
    0 => None,
    _ => Some(res.into()),
  }
}

fn normalize(text: String) -> String {
  let text = text.as_str().replace("&nbsp;", " ");

  let mut chars = text.chars().peekable();
  let mut whitespaces_only = true;
  let mut res = String::new();

  while let Some(c) = chars.next() {
    let next = chars.peek();
    if whitespaces_only && !c.is_whitespace() {
      whitespaces_only = false;
    }

    if c.is_whitespace() && ((next.is_some() && next.unwrap().is_whitespace()) || next.is_none()) {
      continue;
    }

    match c {
      '\n' => (),
      c if c.is_whitespace() => res.push(' '),
      c => res.push(c),
    }
  }

  res
}
