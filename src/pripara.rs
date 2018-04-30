extern crate kuchiki;
extern crate reqwest;
extern crate selectors;
extern crate std;

pub fn update_all() {
    std::fs::create_dir_all("pripara").expect("Failed to create pripara directory");

    for pref in fetch_prefs() {
        let uri = format!("http://pripara.jp/shop/search_list?pref_name={}", pref);
        info!("GET {}", uri);
        let mut resp = reqwest::get(&uri).unwrap();
        use self::kuchiki::traits::TendrilSink;

        info!("{}", resp.status());
        let body = resp.text().unwrap();
        let document = kuchiki::parse_html().one(body);

        let mut shops = vec![];
        for shop_node in document.select("div.h2Tbl").unwrap() {
            if let Some(shop) = extract_shop(shop_node) {
                shops.push(shop);
            }
        }

        let mut csv_writer = super::CsvWriter::new(format!("pripara/{}.csv", pref))
            .expect("Failed to open CSV file");
        csv_writer
            .write_header()
            .expect("Failed to write CSV header");
        for shop in shops {
            csv_writer
                .write_shop(&shop)
                .expect("Unable to write shops to file");
        }
    }
}

fn fetch_prefs() -> Vec<String> {
    let uri = "http://pripara.jp/shop/search_list";

    info!("GET {}", uri);
    let mut resp = reqwest::get(uri).unwrap();
    use self::kuchiki::traits::TendrilSink;

    info!("{}", resp.status());
    let body = resp.text().unwrap();
    let document = kuchiki::parse_html().one(body);

    let mut prefs = vec![];
    for option_node in document.select("select[name=pref_name] option").unwrap() {
        let element = option_node.as_node().as_element().unwrap();
        if let Some(value) = element.attributes.borrow().get("value") {
            if !value.is_empty() {
                prefs.push(value.to_owned());
            }
        }
    }
    prefs
}

fn extract_shop(shop_node: kuchiki::NodeDataRef<kuchiki::ElementData>) -> Option<super::Shop> {
    use self::selectors::Element;
    let name = shop_node.text_contents();
    shop_node.next_sibling_element().and_then(|p| {
        let address = p.text_contents();
        Some(super::Shop {
            name: name.trim().to_owned(),
            address: address.trim().to_owned(),
            units: 0, // XXX: No data?
        })
    })
}
