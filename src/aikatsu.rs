extern crate futures;
extern crate hyper;
extern crate kuchiki;
extern crate selectors;
extern crate std;
extern crate tokio_core;

pub fn update_all() {
    std::fs::create_dir_all("aikatsu").expect("Failed to create aikatsu directory");

    let mut core = tokio_core::reactor::Core::new().expect("Failed to initialize tokio reactor");
    let client = hyper::Client::new(&core.handle());
    for pref in 1..48 {
        let mut csv_writer = super::CsvWriter::new(format!("aikatsu/{:02}.csv", pref))
            .expect("Failed to open CSV file");
        csv_writer.write_header().expect(
            "Failed to write CSV header",
        );

        for page in 1.. {
            use self::futures::Future;

            let uri = format!(
                "http://www.aikatsu.com/stars/playshop/list.php?p={}&pref={:02}",
                page,
                pref
            ).parse()
                .expect("Failed to parse playshop URL");
            info!("GET {}", uri);
            let work = client.get(uri).and_then(|res| {
                use self::futures::Stream;
                use self::kuchiki::traits::TendrilSink;
                use std::borrow::Borrow;

                info!("{}", res.status());
                res.body().concat2().and_then(|body| {
                    let document =
                        kuchiki::parse_html().one(String::from_utf8_lossy(&body).borrow());

                    let mut shops = vec![];
                    for shop_node in document.select("table.shoplist_resultlist").unwrap() {
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
        .and_then(|name_node| {
            Some(name_node.text_contents().trim().to_owned())
        })
}

fn extract_shop_address(shop_node: &kuchiki::NodeRef) -> Option<String> {
    shop_node.select("a.btn_map").unwrap().next().and_then(
        |map_node| {
            use self::selectors::Element;
            map_node.parent_element().and_then(|parent_node| {
                parent_node.as_node().first_child().and_then(|text_node| {
                    Some(text_node.text_contents().trim().to_owned())
                })
            })
        },
    )
}

fn extract_shop_units(shop_node: &kuchiki::NodeRef) -> usize {
    shop_node
        .select("th.period ~ td .titlenum dl")
        .unwrap()
        .count()
}
