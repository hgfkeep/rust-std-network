extern crate env_logger;
extern crate log;
use log::info;
use serde::de::{self, MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_test::{assert_de_tokens, Token};
use std::fmt;


/// 自定义序列化的对象
#[derive(Debug, PartialEq)]
struct KubeConfig {
    port: u8,
    healthz_port: u8,
    max_pods: u8,
}

/// 实现自定义序列化
impl Serialize for KubeConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 序列化为结构体序列化
        let mut state = serializer.serialize_struct("KubeConfig", 3)?;

        // 序列化结构体的每个成员
        state.serialize_field("port", &self.port)?;
        state.serialize_field("healthz_port", &self.healthz_port)?;
        state.serialize_field("max_pods", &self.max_pods)?;

        //标记序列化结束
        state.end()
    }
}

/// 自定义反序列化
impl<'de> Deserialize<'de> for KubeConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        /// 结构体的每个成员， 反序列化结构体时，需要针对每个成员处理
        enum Field {
            Port,
            HealthzPort,
            MaxPods,
        };

        /// 对Field 实现反序列化
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {

                /// 实现反序列化时的Visitor trait
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`port` or `healthz_port`or `max_pods`")
                    }

                    /// 针对结构体序列化结果中的每个field的名字，反序列化出结构体的成员
                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "port" => Ok(Field::Port),
                            "healthz_port" => Ok(Field::HealthzPort),
                            "max_pods" => Ok(Field::MaxPods),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                /// 反序列化成员字段
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct KubeConfigVisitor;

        impl<'de> Visitor<'de> for KubeConfigVisitor {
            type Value = KubeConfig;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct KubeConfig")
            }

            /// 反序列化结构体的成员名和成员值
            fn visit_map<V>(self, mut map: V) -> Result<KubeConfig, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut port = None;
                let mut healthz_port = None;
                let mut max_pods = None;

                // 对于每个成员名和值，分别反序列化
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Port => {
                            if port.is_some() {
                                return Err(de::Error::duplicate_field("port"));
                            }
                            port = Some(map.next_value()?);
                        }
                        Field::HealthzPort => {
                            if healthz_port.is_some() {
                                return Err(de::Error::duplicate_field("healthz_port"));
                            }
                            healthz_port = Some(map.next_value()?);
                        }
                        Field::MaxPods => {
                            if max_pods.is_some() {
                                return Err(de::Error::duplicate_field("max_pods"));
                            }
                            max_pods = Some(map.next_value()?);
                        }
                    }
                }

                let port = port.ok_or_else(|| de::Error::missing_field("port"))?;
                let hport = healthz_port.ok_or_else(|| de::Error::missing_field("healthz_port"))?;
                let max = max_pods.ok_or_else(|| de::Error::missing_field("max_pods"))?;
                
                //构造结构体，
                Ok(KubeConfig {
                    port: port,
                    healthz_port: hport,
                    max_pods: max,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["port", "healthz_port", "max_pods"];
        deserializer.deserialize_struct("KubeConfig", FIELDS, KubeConfigVisitor)
    }
}

#[test]
fn test_ser_de() {
    let kubeconfig = KubeConfig {
        port: 80,
        healthz_port: 90,
        max_pods: 10,
    };

    assert_de_tokens(
        &kubeconfig,
        &[
            Token::Struct {
                name: "KubeConfig",
                len: 3,
            },
            Token::Str("port"),
            Token::U8(80),
            Token::Str("healthz_port"),
            Token::U8(90),
            Token::Str("max_pods"),
            Token::U8(10),
            Token::StructEnd,
        ],
    );
}

fn main() {
    env_logger::init();
    let kubeconfig = KubeConfig {
        port: 80,
        healthz_port: 90,
        max_pods: 10,
    };

    let serialized = serde_json::to_string(&kubeconfig).expect("serialize error");
    info!("serialized: {}", serialized);

    let config: KubeConfig = serde_json::from_str(&serialized.as_str()).expect("deserialize error");
    info!("deserialized: {:?}", config);
}
