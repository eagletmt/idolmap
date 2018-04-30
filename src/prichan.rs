extern crate kuchiki;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate std;
extern crate url;

pub fn update_all() {
    std::fs::create_dir_all("prichan").expect("Failed to create prichan directory");

    for (pref, pcode) in fetch_prefs() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        let timestamp = now.as_secs() * 1000 + now.subsec_nanos() as u64 / 1000000;
        let uri = format!(
            "https://prichan.jp/shop/data/pref{}.js?_={}",
            pcode, timestamp
        );
        info!("GET {}", uri);
        let mut resp = reqwest::get(&uri).unwrap();
        info!("{}", resp.status());
        let body: PrichanResponse = resp.json().unwrap();

        let shops: Vec<_> = body.shops
            .into_iter()
            .map(|shop| super::Shop {
                name: shop.name,
                address: shop.address,
                units: 0,
            })
            .collect();

        let mut csv_writer = super::CsvWriter::new(format!("prichan/{}.csv", pref))
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

fn fetch_prefs() -> Vec<(String, String)> {
    let uri = url::Url::parse("https://prichan.jp/shop/index.html").unwrap();

    info!("GET {}", uri);
    let mut resp = reqwest::get(uri.clone()).unwrap();
    info!("{}", resp.status());
    let body = resp.text().unwrap();

    use self::kuchiki::traits::TendrilSink;
    let document = kuchiki::parse_html().one(body);

    let mut prefs = vec![];
    for a_node in document.select(".col.-c4 a").unwrap() {
        let element = a_node.as_node().as_element().unwrap();
        if let Some(href) = element.attributes.borrow().get("href") {
            let u = uri.join(href).unwrap();
            let kv = u.query_pairs()
                .find(|&(ref key, _)| key == "pcode")
                .expect("pcode parameter is missing");
            prefs.push((a_node.text_contents().trim().to_owned(), kv.1.into_owned()));
        }
    }
    prefs
}

#[derive(Debug, Deserialize)]
struct PrichanResponse {
    #[serde(rename = "sList")]
    shops: Vec<PrichanShop>,
}

#[derive(Debug, Deserialize)]
struct PrichanShop {
    name: String,
    address: String,
}
