use std::borrow::Cow;

use tl::{Node, NodeHandle, ParseError, Parser, ParserOptions};

pub fn parse_html(html: &str) -> Result<(), ParseError> {
  let dom = tl::parse(html, ParserOptions::default())?;
  let parser = dom.parser();
  let table = dom
    .query_selector("table")
    .expect("Unable to perform selection")
    .last()
    .unwrap();

  parse_table(table.get(parser).unwrap().inner_html(parser))?;

  Ok(())
}

fn parse_table(html: Cow<str>) -> Result<(), ParseError> {
  let dom = tl::parse(&html, ParserOptions::default())?;
  let parser = dom.parser();
  let trs = dom.query_selector("tr").expect("Unable to select tr");

  for tr in trs.map(|tr| tr.get(parser).unwrap()).skip(2) {
    for td in tr
      .children()
      .expect("Unable to get children")
      .top()
      .iter()
      .filter_map(|handle| get_inner_text(parser, handle))
    {
      println!("{:?}", td);
    }
    println!()
  }
  Ok(())
}

/*
     .iter()
     .filter(|tag| tag.children().map(|child| child.top().len() < 2).unwrap_or(false))
     .filter_map(|node| {
       node
         .children()?
         .top()
         .iter()
         .find(|node| {
           node
             .get(parser)
             .unwrap()
             .find_node(parser, &mut |node| node.as_tag().map_or(false, |tag| tag.name() == "span"))
             .is_some()
         })
         .map(|node| node.get(parser))
*/

fn get_inner_text(parser: &Parser, node: &NodeHandle) -> Option<String> {
  let res = node.get(parser)?.inner_text(parser);
  let res = res.trim();
  match res.len() {
    0 => None,
    _ => Some(res.into()),
  }
}
