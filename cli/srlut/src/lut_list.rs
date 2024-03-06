use std::str::FromStr;

use serde::{
    de::{self, Visitor},
    ser::SerializeSeq,
    Serialize,
};
use solana_sdk::pubkey::Pubkey;

pub struct LutList(pub Vec<Pubkey>);

impl Serialize for LutList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for pk in self.0.iter() {
            seq.serialize_element(&pk.to_string())?;
        }
        seq.end()
    }
}

struct LutListDeser;

impl<'de> Visitor<'de> for LutListDeser {
    type Value = LutList;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("List of base58-encoded pubkeys")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut res = vec![];
        while let Some(pk_str) = seq.next_element()? {
            res.push(Pubkey::from_str(pk_str).map_err(|e| de::Error::custom(format!("{e}")))?);
        }
        Ok(LutList(res))
    }
}
