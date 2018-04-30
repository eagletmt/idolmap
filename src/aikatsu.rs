extern crate kuchiki;
extern crate reqwest;
extern crate selectors;
extern crate std;
extern crate url;

pub fn update_all() {
    std::fs::create_dir_all("aikatsu").expect("Failed to create aikatsu directory");

    let prefs = fetch_prefs();

    for (pref_id, pref) in prefs {
        let base_uri = format!(
            "http://www.aikatsu.com/friends/playshop/list.php?p=1&pref={:02}",
            pref_id
        );
        let base_url = url::Url::parse(&base_uri).unwrap();
        let mut resp = reqwest::get(&base_uri).unwrap();
        info!("{}", resp.status());
        let body = resp.text().unwrap();

        use self::kuchiki::traits::TendrilSink;
        let document = kuchiki::parse_html().one(body);

        let last_page_node = document
            .select("div.paginator > a")
            .unwrap()
            .last()
            .expect("No a element in paginator");
        let last_page_element = last_page_node.as_node().as_element().unwrap();
        let attributes = last_page_element.attributes.borrow();
        let last_page = if let Some(href) = attributes.get("href") {
            let last_page_url = base_url.join(href).expect("Failed to parse last_page URL");
            let pair = last_page_url
                .query_pairs()
                .find(|p| p.0 == "p")
                .expect("Failed to find p parameter");
            pair.1.parse().expect("Non-integer p parameter")
        } else {
            // No pagination
            1
        };

        let mut shops = vec![];
        for page in 0..last_page {
            let uri = format!(
                "http://www.aikatsu.com/friends/playshop/list.php?p={}&pref={:02}",
                page + 1,
                pref_id
            );
            info!("GET {}", uri);
            let mut resp = reqwest::get(&uri).unwrap();
            info!("{}", resp.status());
            let body = resp.text().unwrap();
            let document = kuchiki::parse_html().one(body);

            for shop_node in document.select("table.shoplist_resultlist").unwrap() {
                if let Some(shop) = extract_shop(shop_node) {
                    shops.push(shop);
                }
            }
        }

        let mut csv_writer = super::CsvWriter::new(format!("aikatsu/{}.csv", pref))
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

fn fetch_prefs() -> std::collections::HashMap<i32, String> {
    let uri = "http://www.aikatsu.com/friends/playshop/list.php?p=1";
    info!("GET {}", uri);
    let mut resp = reqwest::get(uri).unwrap();
    info!("{}", resp.status());
    let body = resp.text().unwrap();

    use self::kuchiki::traits::TendrilSink;
    let document = kuchiki::parse_html().one(body);

    let mut prefs = std::collections::HashMap::new();
    for option_node in document.select("#pref_id option").unwrap() {
        let element = option_node.as_node().as_element().unwrap();
        if let Some(value) = element.attributes.borrow().get("value") {
            if !value.is_empty() {
                if let Some(label) = element.attributes.borrow().get("label") {
                    prefs.insert(value.parse().expect("non-integer value"), label.to_owned());
                }
            }
        }
    }
    prefs
}

fn extract_shop(shop_node: kuchiki::NodeDataRef<kuchiki::ElementData>) -> Option<super::Shop> {
    let noderef = shop_node.as_node();
    extract_shop_name(noderef).and_then(|name| {
        extract_shop_address(noderef).and_then(|address| {
            Some(super::Shop {
                name: name,
                address: address,
                units: extract_shop_units(noderef),
            })
        })
    })
}

fn extract_shop_name(shop_node: &kuchiki::NodeRef) -> Option<String> {
    shop_node
        .select("th.list-name ~ td")
        .unwrap()
        .next()
        .and_then(|name_node| Some(name_node.text_contents().trim().to_owned()))
}

fn extract_shop_address(shop_node: &kuchiki::NodeRef) -> Option<String> {
    shop_node
        .select("a.btn_map")
        .unwrap()
        .next()
        .and_then(|map_node| {
            use self::selectors::Element;
            map_node.parent_element().and_then(|parent_node| {
                parent_node
                    .as_node()
                    .first_child()
                    .and_then(|text_node| Some(text_node.text_contents().trim().to_owned()))
            })
        })
}

fn extract_shop_units(shop_node: &kuchiki::NodeRef) -> usize {
    shop_node
        .select("th.period ~ td .titlenum dl")
        .unwrap()
        .count()
}
