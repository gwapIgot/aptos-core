// @generated
impl serde::Serialize for GetAccountModuleRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.account_address.is_empty() {
            len += 1;
        }
        if !self.module_name.is_empty() {
            len += 1;
        }
        if self.ledger_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.api.v2.GetAccountModuleRequest", len)?;
        if !self.account_address.is_empty() {
            struct_ser.serialize_field("accountAddress", &self.account_address)?;
        }
        if !self.module_name.is_empty() {
            struct_ser.serialize_field("moduleName", &self.module_name)?;
        }
        if let Some(v) = self.ledger_version.as_ref() {
            struct_ser.serialize_field("ledgerVersion", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAccountModuleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "account_address",
            "accountAddress",
            "module_name",
            "moduleName",
            "ledger_version",
            "ledgerVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccountAddress,
            ModuleName,
            LedgerVersion,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "accountAddress" | "account_address" => Ok(GeneratedField::AccountAddress),
                            "moduleName" | "module_name" => Ok(GeneratedField::ModuleName),
                            "ledgerVersion" | "ledger_version" => Ok(GeneratedField::LedgerVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAccountModuleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.api.v2.GetAccountModuleRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetAccountModuleRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut account_address__ = None;
                let mut module_name__ = None;
                let mut ledger_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AccountAddress => {
                            if account_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountAddress"));
                            }
                            account_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::ModuleName => {
                            if module_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("moduleName"));
                            }
                            module_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::LedgerVersion => {
                            if ledger_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ledgerVersion"));
                            }
                            ledger_version__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(GetAccountModuleRequest {
                    account_address: account_address__.unwrap_or_default(),
                    module_name: module_name__.unwrap_or_default(),
                    ledger_version: ledger_version__,
                })
            }
        }
        deserializer.deserialize_struct("aptos.api.v2.GetAccountModuleRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAccountModuleResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.module.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.api.v2.GetAccountModuleResponse", len)?;
        if let Some(v) = self.module.as_ref() {
            match v {
                get_account_module_response::Module::MoveModuleBytecode(v) => {
                    struct_ser.serialize_field("moveModuleBytecode", v)?;
                }
                get_account_module_response::Module::MoveModuleBytecodeRaw(v) => {
                    struct_ser.serialize_field("moveModuleBytecodeRaw", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAccountModuleResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "move_module_bytecode",
            "moveModuleBytecode",
            "move_module_bytecode_raw",
            "moveModuleBytecodeRaw",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MoveModuleBytecode,
            MoveModuleBytecodeRaw,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "moveModuleBytecode" | "move_module_bytecode" => Ok(GeneratedField::MoveModuleBytecode),
                            "moveModuleBytecodeRaw" | "move_module_bytecode_raw" => Ok(GeneratedField::MoveModuleBytecodeRaw),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAccountModuleResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.api.v2.GetAccountModuleResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetAccountModuleResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut module__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MoveModuleBytecode => {
                            if module__.is_some() {
                                return Err(serde::de::Error::duplicate_field("moveModuleBytecode"));
                            }
                            module__ = map.next_value::<::std::option::Option<_>>()?.map(get_account_module_response::Module::MoveModuleBytecode)
;
                        }
                        GeneratedField::MoveModuleBytecodeRaw => {
                            if module__.is_some() {
                                return Err(serde::de::Error::duplicate_field("moveModuleBytecodeRaw"));
                            }
                            module__ = map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| get_account_module_response::Module::MoveModuleBytecodeRaw(x.0));
                        }
                    }
                }
                Ok(GetAccountModuleResponse {
                    module: module__,
                })
            }
        }
        deserializer.deserialize_struct("aptos.api.v2.GetAccountModuleResponse", FIELDS, GeneratedVisitor)
    }
}
