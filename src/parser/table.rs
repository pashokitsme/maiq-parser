use std::{
  borrow::Cow,
  time::{Duration, Instant},
};

use anyhow::anyhow;
use tl::{NodeHandle, Parser, ParserOptions};

pub struct Table {
  pub rows: Vec<Vec<String>>,
  pub elapsed: Duration,
}

pub fn parse_html(html: &str) -> anyhow::Result<Table> {
  let now = Instant::now();
  let dom = tl::parse(html, ParserOptions::default())?;
  let parser = dom.parser();
  let table = dom
    .query_selector("table")
    .ok_or_else(|| anyhow!("Unable to select `table"))?
    .last()
    .ok_or_else(|| anyhow!("Unable to select table"))?;

  let values = parse_table(table.get(parser).unwrap().inner_html(parser))?;
  let elapsed = now.elapsed();
  Ok(Table { rows: values, elapsed })
}

fn parse_table(html: Cow<str>) -> anyhow::Result<Vec<Vec<String>>> {
  let dom = tl::parse(&html, ParserOptions::default())?;
  let parser = dom.parser();
  let trs = dom
    .query_selector("tr")
    .ok_or_else(|| anyhow!("Unable to select `tr`"))?;

  let table = trs
    .map(|tr| tr.get(parser).unwrap())
    .map(|tr| {
      tr.children()
        .unwrap()
        .top()
        .iter()
        .filter_map(|handle| get_inner_text(parser, handle))
        .map(normalize)
        .collect::<Vec<String>>()
    })
    .filter(|x| !x.iter().all(|x| x.is_empty()))
    .collect::<Vec<Vec<String>>>();

  Ok(table)
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
  let text = text.as_str().replace("&nbsp;", " ").replace("&quot;", "\"");
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
