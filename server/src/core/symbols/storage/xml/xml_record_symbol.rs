use std::ops::Range;
use crate::{core::symbols::{SymbolTable, storage::xml::xml_field_symbol::XmlFieldName}, utils::HashMap};

use ruff_text_size::TextRange;

use crate::{constants::OYarn, core::symbols::symbol_keys::{SymbolKey, XmlFieldKey}};

#[derive(Debug)]
pub struct XmlRecordSymbol {
    pub is_external: bool,
    pub model: (OYarn, Range<usize>),
    pub xml_id: Option<OYarn>,
    pub (in crate::core::symbols::storage) fields: HashMap<OYarn, XmlFieldKey>,
    pub range: TextRange,
    declared_model: Option<OYarn>,

    parent: SymbolKey,
}

impl XmlRecordSymbol {
    pub fn new(
        model: (OYarn, Range<usize>),
        xml_id: Option<OYarn>,
        range: TextRange,
        parent: SymbolKey,
        is_external: bool,
    ) -> Self {
        Self {
            model,
            xml_id,
            fields: HashMap::default(),
            range,
            parent,
            is_external,
            declared_model: None,
        }
    }

    pub fn parent(&self) -> SymbolKey {
        self.parent
    }

    pub fn children(&self) -> Vec<SymbolKey> {
        self.fields.values().map(|k| SymbolKey::XmlField(*k)).collect()
    }

    pub fn fields(&self) -> &HashMap<OYarn, XmlFieldKey> {
        &self.fields
    }

    pub fn set_declared_model(&mut self, model: OYarn) {
        self.declared_model = Some(model);
    }

    pub fn get_declared_model(&self) -> Option<&OYarn> {
        self.declared_model.as_ref()
    }

    /// Get the text of a field in this record with the given name
    pub fn get_field_text(&self, field_name: XmlFieldName, symbol_table: &SymbolTable) -> Option<String> {
        let &field_key = self.fields().get(field_name.as_str())?;
        let field_symbol = &symbol_table[field_key];
        field_symbol.text.clone()
    }
}
