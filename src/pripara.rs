extern crate futures;
extern crate hyper;
extern crate kuchiki;
extern crate selectors;
extern crate std;
extern crate tokio_core;

#[derive(Debug)]
struct Pref {
    id: usize,
    name: String,
}

pub fn update_all() {
    std::fs::create_dir_all("pripara").expect("Failed to create pripara directory");

    let mut core = tokio_core::reactor::Core::new().expect("Failed to initialize tokio reactor");
    let client = hyper::Client::new(&core.handle());
    for pref in fetch_prefs(&mut core, &client) {
        use self::futures::Future;

        let mut csv_writer = super::CsvWriter::new(format!("pripara/{:02}.csv", pref.id))
            .expect("Failed to open CSV file");
        csv_writer.write_header().expect(
            "Failed to write CSV header",
        );

        let uri = format!("http://pripara.jp/shop/search_list?pref_name={}", pref.name)
            .parse()
            .expect("Failed to parse search_list URL");
        info!("GET {}", uri);
        let work = client.get(uri).and_then(|res| {
            use self::futures::Stream;
            use self::kuchiki::traits::TendrilSink;
            use std::borrow::Borrow;

            info!("{}", res.status());
            res.body().concat2().and_then(|body| {
                let document = kuchiki::parse_html().one(String::from_utf8_lossy(&body).borrow());

                let mut shops = vec![];
                for shop_node in document.select("div.h2Tbl").unwrap() {
                    if let Some(shop) = extract_shop(shop_node) {
                        shops.push(shop);
                    }
                }
                Ok(shops)
            })
        });
        let shops = core.run(work).expect("Failed to run tokio event loop");
        if shops.is_empty() {
            break;
        }

        for shop in shops {
            csv_writer.write_shop(&shop).expect(
                "Unable to write shops to file",
            );
        }
    }
}

fn fetch_prefs(
    core: &mut tokio_core::reactor::Core,
    client: &hyper::Client<hyper::client::HttpConnector>,
) -> Vec<Pref> {
    use self::futures::Future;

    let uri = "http://pripara.jp/shop/search_list".parse().expect(
        "Failed to parse search_list URL",
    );

    info!("GET {}", uri);
    let work = client.get(uri).and_then(|res| {
        use self::futures::Stream;
        use self::kuchiki::traits::TendrilSink;
        use std::borrow::Borrow;

        info!("{}", res.status());
        res.body().concat2().and_then(|body| {
            let document = kuchiki::parse_html().one(String::from_utf8_lossy(&body).borrow());

            let mut prefs = vec![];
            for (i, option_node) in document
                .select("select[name=pref_name] option")
                .unwrap()
                .enumerate()
            {
                let element = option_node.as_node().as_element().unwrap();
                if let Some(value) = element.attributes.borrow().get("value") {
                    if !value.is_empty() {
                        prefs.push(Pref {
                            id: i,
                            name: value.to_owned(),
                        });
                    }
                }
            }
            Ok(prefs)
        })
    });
    core.run(work).unwrap()
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
