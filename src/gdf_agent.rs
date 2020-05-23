use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json;
#[allow(unused_imports)]
use crate::errors::Result;

// see https://serde.rs/field-attrs.html
#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    
    #[serde(rename = "isOverridable")]
    pub is_overridable: bool,
    
    #[serde(rename = "isEnum")]
    pub is_enum: bool,
    
    #[serde(rename = "isRegexp")]
    pub is_regexp: bool,
    
    #[serde(rename = "automatedExpansion")]
    pub automated_expansion: bool,

    #[serde(rename = "allowFuzzyExtraction")]
    pub allow_fuzzy_extraction: bool
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_deser_ser() -> Result<()> {

        let entity_str = r#"
        {
            "id": "ed3dad98-49c6-4370-9f7e-0c6648d99820",
            "name": "additional",
            "isOverridable": true,
            "isEnum": true,
            "isRegexp": false,
            "automatedExpansion": false,
            "allowFuzzyExtraction": false
        }
        "#;        
        let entity: Entity = serde_json::from_str(entity_str)?;
        assert_eq!(entity.id,  "ed3dad98-49c6-4370-9f7e-0c6648d99820");
        assert_eq!(entity.name,  "additional");
        assert_eq!(entity.is_overridable,  true);
        assert_eq!(entity.is_enum,  true);
        assert_eq!(entity.is_regexp,  false);
        assert_eq!(entity.automated_expansion,  false);
        assert_eq!(entity.allow_fuzzy_extraction,  false);

        let serialized_str = serde_json::to_string(&entity).unwrap();
        let serialized_str_expected = r#"{"id":"ed3dad98-49c6-4370-9f7e-0c6648d99820","name":"additional","isOverridable":true,"isEnum":true,"isRegexp":false,"automatedExpansion":false,"allowFuzzyExtraction":false}"#;
        assert_eq!(serialized_str,  serialized_str_expected);
        Ok(())
    }
}