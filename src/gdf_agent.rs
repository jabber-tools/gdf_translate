use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json;
#[allow(unused_imports)]
use crate::errors::Result;

// TBD: https://serde.rs/field-attrs.html
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub isOverridable: bool,
    pub isEnum: bool,
    pub isRegexp: bool,
    pub automatedExpansion: bool,
    pub allowFuzzyExtraction: bool
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_from_str() -> Result<()> {

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
        assert_eq!(entity.isOverridable,  true);
        assert_eq!(entity.isEnum,  true);
        assert_eq!(entity.isRegexp,  false);
        assert_eq!(entity.automatedExpansion,  false);
        assert_eq!(entity.allowFuzzyExtraction,  false);
        
        Ok(())
    }
}