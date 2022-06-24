use crate::EXCHANGE_RATE_API_CONSUMER;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Error};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[typetag::serde()]
pub trait Currency: Display {
    fn btc_value(&self) -> f64;
}
// #[typetag::serde()]
// #[async_trait]
// pub trait Currency: Display {
//     async fn btc_value(&self, exchange_rate_api_consumer: &mut ExchangeRateProvider<ApiConsumer>) -> f64;
// }

pub struct Currencies;

impl Currencies {
    pub fn parse(input: &str) -> Result<Box<dyn Currency>, Error> {
        if let Ok(btc) = BitcoinUnit::from_str(input) {
            return Ok(Box::new(btc));
        }

        if let Ok(fiat) = Fiat::from_str(input) {
            return Ok(Box::new(fiat));
        }

        Err(Default::default())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, EnumString, Display)]
#[strum(ascii_case_insensitive, serialize_all = "UPPERCASE")]
pub enum BitcoinUnit {
    BTC,  // bitcoin
    MBTC, // milli-bitcoin
    BITS, // μBTC, micro-bitcoin
    SAT,  // satoshi
    MSAT, // milli-satoshi
}

#[typetag::serde]
#[async_trait]
impl Currency for BitcoinUnit {
    fn btc_value(&self) -> f64 {
        match &self {
            BitcoinUnit::BTC => 1.0,
            BitcoinUnit::MBTC => 0.001,
            BitcoinUnit::BITS => 0.000_001,
            BitcoinUnit::SAT => 0.000_000_01,
            BitcoinUnit::MSAT => 0.000_000_000_01,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, EnumString, Display)]
#[strum(ascii_case_insensitive, serialize_all = "UPPERCASE")]
pub enum Fiat {
    ARS,
    AUD,
    BRL,
    CAD,
    CHF,
    CLP,
    CNY,
    CZK,
    DKK,
    EUR,
    GBP,
    HKD,
    HRK,
    HUF,
    INR,
    ISK,
    JPY,
    KRW,
    NZD,
    PLN,
    RON,
    RUB,
    SEK,
    SGD,
    THB,
    TRY,
    TWD,
    USD,
}

#[typetag::serde]
impl Currency for Fiat {
    fn btc_value(&self) -> f64 {
        unsafe { EXCHANGE_RATE_API_CONSUMER.btc_value(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_correct_fiat_currency() {
        let currency_lowercase = Currencies::parse("usd").unwrap();
        let currency_capitalized = Currencies::parse("Usd").unwrap();
        let currency_uppercase = Currencies::parse("USD").unwrap();

        assert_eq!(currency_lowercase.to_string(), "USD");
        assert_eq!(currency_capitalized.to_string(), "USD");
        assert_eq!(currency_uppercase.to_string(), "USD");
    }

    #[test]
    fn should_return_correct_bitcoin_denomination() {
        let currency_lowercase = Currencies::parse("btc").unwrap();
        let currency_capitalized = Currencies::parse("Btc").unwrap();
        let currency_uppercase = Currencies::parse("BTC").unwrap();

        assert_eq!(currency_lowercase.to_string(), "BTC");
        assert_eq!(currency_capitalized.to_string(), "BTC");
        assert_eq!(currency_uppercase.to_string(), "BTC");
    }

    #[test]
    fn incorrect_use_should_return_error() {
        let currency_empty_string = Currencies::parse("");
        let currency_non_existant = Currencies::parse("non-existant");

        assert!(currency_empty_string.is_err());
        assert!(currency_non_existant.is_err());
    }
}
