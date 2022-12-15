use aho_corasick::AhoCorasickBuilder;
use scraper::Html;
use std::fmt::Display;

pub enum Fetch {
  Today,
  Tomorrow,
}

impl Display for Fetch {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let url = match self {
      Fetch::Tomorrow => "https://rsp.chemk.org/4korp/tomorrow.htm",
      Fetch::Today => "https://rsp.chemk.org/4korp/today.htm",
    };
    f.write_str(url)
  }
}

pub async fn fetch(fetch_mode: Fetch) -> Result<String, reqwest::Error> {
  reqwest::get(fetch_mode.to_string())
    .await?
    .text_with_charset("windows-1251")
    .await
}

//todo: use tl crate (currently query selector not working)
pub fn parse(html: &str) {
  let table = table_extract::Table::find_first(html).unwrap();
  let corasick = AhoCorasickBuilder::new()
    .ascii_case_insensitive(true)
    .build(&["  ", " ", "\n"]);
  let replace_with = &[" ", "", ""];
  for row in table.iter().skip(3) {
    for cell in row {
      let cell = Html::parse_fragment(cell.as_str());
      let cell = corasick.replace_all(&cell.root_element().text().collect::<String>(), replace_with);
      if cell.is_empty() {
        continue;
      }
      print!("{} | ", cell);
    }
    println!()
  }
  /*
    let watch = Stopwatch::start_new();
    let doc = Html::parse_document(&html);
    let sel = Selector::parse("body > div > div > table > tbody").unwrap();
    //? body > div > div > table > tbody > tr:nth-child(33) > td:nth-child(1) > p > b > span
    let inner_sel = Selector::parse("tr > td > p > b > span").unwrap();
    for el in doc.select(&sel) {
      let mut group_index = 0usize;
      for group_el in el.select(&inner_sel).skip(5) {
        let text = group_el.text().collect::<String>();
        if !is_matches_group(&text) {
          continue;
        }
        info!("{}. {}", group_index, text);
        group_index += 1;
      }
    }

    info!("Parsing took: {}ms", watch.elapsed_ms())
  */
  /*
  let dom = tl::parse(html.as_str(), ParserOptions::default()).unwrap();
  let parser = dom.parser();
  let elements = dom.query_selector("table.MsoNormalTable > ").unwrap();
  let count = elements.clone().count();
  println!("{}", count);
  for el in elements {
    let inner = el.get(parser).unwrap().inner_html(parser);
    println!("{}", &*inner);
    let dom = tl::parse(&*inner, ParserOptions::default()).unwrap();
    let inner_parser = dom.parser();
    let groups = dom.query_selector("td[rowspan] > p > b > span");

    if groups.is_none() {
      continue;
    }
    let groups = groups.unwrap();

    for group in groups {
      println!("{}", group.get(&inner_parser).unwrap().inner_html(&parser));
      let group = group.get(&inner_parser);
      if group.is_none() {
        continue;
      }
      let group = group.unwrap();
      if group.inner_text(&inner_parser).len() < 4 {
        continue;
      }
    }
  }
  */
}

fn is_matches_group(source: &String) -> bool {
  source.chars().rev().take(2).all(|c| c >= '0' && c <= '9')
    && source.chars().rev().nth(2).unwrap_or_default() == '-'
    && source
      .chars()
      .rev()
      .skip(4)
      .all(|c| (c >= 'а' && c <= 'я') || (c >= 'А' || c <= 'Я'))
}
