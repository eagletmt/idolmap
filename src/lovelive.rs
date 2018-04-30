extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate std;

#[derive(Debug, Deserialize)]
struct Locationlist {
    #[serde(rename = "DATA")]
    data: Vec<ShopData>,
}

#[derive(Debug, Deserialize)]
struct ShopData {
    #[serde(rename = "TNAME")]
    tname: String,
    #[serde(rename = "PREF")]
    pref: String,
    #[serde(rename = "ADDR")]
    addr: String,
    #[serde(rename = "CNT", deserialize_with = "str_or_int")]
    cnt: usize,
}

fn str_or_int<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct Visitor;
    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = usize;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an integer or a string")
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value as usize)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match value.parse() {
                Ok(v) => Ok(v),
                Err(_) => Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(value),
                    &"integer value",
                )),
            }
        }
    }

    deserializer.deserialize_any(Visitor)
}

pub fn update_all() {
    std::fs::create_dir_all("lovelive").expect("Failed to create lovelive directory");

    let client = reqwest::Client::new();

    let uri = "http://www.lovelive-sifac.jp/location_getlocationlist_withcache.php";
    info!("GET {}", uri);
    let mut resp = client.get(uri).send().expect("Failed to send HTTP request");
    info!("{}", resp.status());
    let body = resp.text().expect("Failed to read response body");
    let locationlist: Locationlist =
        serde_json::from_str(&body[1..body.len() - 1]).expect("Failed to deserialize JSON body");

    let mut h = std::collections::HashMap::new();
    for shop_data in locationlist.data {
        let pref = shop_data.pref.clone();
        h.entry(pref).or_insert(vec![]).push(super::Shop {
            name: shop_data.tname,
            address: format!("{}{}", shop_data.pref, shop_data.addr),
            units: shop_data.cnt,
        })
    }
    for (pref, shops) in h {
        let mut csv_writer = super::CsvWriter::new(format!("lovelive/{}.csv", pref))
            .expect("Failed to open CSV file");
        csv_writer
            .write_header()
            .expect("Failed to write CSV header");
        for shop in shops {
            csv_writer
                .write_shop(&shop)
                .expect("Failed to write shop to CSV file");
        }
    }
}
