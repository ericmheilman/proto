mod origin {
    use serde::{de, ser, Deserialize, Deserializer, Serializer};

    fn integer_to_str<S>(input: &i32) -> Result<&str, S::Error>
    where
        S: Serializer,
    {
        if *input == 0 {
            Ok("p2p")
        } else if *input == 1 {
            Ok("radio")
        } else {
            Err(ser::Error::custom(
                "Invalid integer for serializing origin field",
            ))
        }
    }

    pub fn serialize<S>(input: &i32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = integer_to_str::<S>(input)?;
        serializer.serialize_str(s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        if s == "p2p" {
            Ok(0)
        } else if s == "radio" {
            Ok(1)
        } else {
            Err(de::Error::custom(
                "Invalid string for deserializing origin field",
            ))
        }
    }
}

mod reward_type {
    use serde::{de, ser, Deserialize, Deserializer, Serializer};

    fn integer_to_str<S>(input: &i32) -> Result<&str, S::Error>
    where
        S: Serializer,
    {
        if *input == 0 {
            Ok("securities")
        } else if *input == 1 {
            Ok("data_credits")
        } else if *input == 2 {
            Ok("poc_challengees")
        } else if *input == 3 {
            Ok("poc_challengers")
        } else if *input == 4 {
            Ok("poc_witnesses")
        } else if *input == 5 {
            Ok("consensus")
        } else {
            Err(ser::Error::custom(
                "Invalid integer for serializing origin field",
            ))
        }
    }

    pub fn serialize<S>(input: &i32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = integer_to_str::<S>(input)?;
        serializer.serialize_str(s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        if s == "securities" {
            Ok(0)
        } else if s == "data_credits" {
            Ok(1)
        } else if s == "poc_challengees" {
            Ok(2)
        } else if s == "poc_challengers" {
            Ok(3)
        } else if s == "poc_witnesses" {
            Ok(4)
        } else if s == "consensus" {
            Ok(5)
        } else {
            Err(de::Error::custom(
                "Invalid string for deserializing origin field",
            ))
        }
    }
}

pub mod base58 {
    extern crate bs58;
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&bs58::encode(bytes).into_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<&str> = Option::deserialize(deserializer)?;
        if let Some(s) = opt {
            bs58::decode(s).into_vec().map_err(de::Error::custom)
        } else {
            Ok(Vec::new())
        }
    }
}

mod base64 {
    extern crate base64;
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&base64::encode(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        base64::decode(s).map_err(de::Error::custom)
    }
}

mod base64_url {
    extern crate base64;
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&base64::encode_config(bytes, base64::URL_SAFE))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        base64::decode_config(s, base64::URL_SAFE).map_err(de::Error::custom)
    }
}

mod u64_base64 {
    extern crate base64;
    use bytes::{BigEndian, ByteOrder};
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(word: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut vec = Vec::new();
        BigEndian::write_u64_into(&[*word], &mut vec);
        serializer.serialize_str(&base64::encode(vec))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        match base64::decode(s) {
            Ok(vec) => Ok(BigEndian::read_u64(&vec)),
            Err(e) => Err(de::Error::custom(e)),
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/helium.rs"));

pub use blockchain_txn::Txn;
pub use prost::Message;
#[cfg(feature = "services")]
pub mod services {
    pub mod router {
        pub use crate::router_client::RouterClient as Client;
    }
    pub mod validator {
        pub use crate::validator_client::ValidatorClient as Client;
    }

    pub use tonic::transport::*;
}
