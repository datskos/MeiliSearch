use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};

use crate::{FieldsMap, FieldId, SResult, Error, IndexedPos};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Schema {
    fields_map: FieldsMap,

    identifier: FieldId,
    ranked: HashSet<FieldId>,
    displayed: HashSet<FieldId>,

    indexed: Vec<FieldId>,
    indexed_map: HashMap<FieldId, IndexedPos>,

    index_new_fields: bool,
}

impl Schema {

    pub fn with_identifier<S: Into<String>>(name: S) -> Schema {
        let mut fields_map = FieldsMap::default();
        let field_id = fields_map.insert(name.into()).unwrap();

        Schema {
            fields_map,
            identifier: field_id,
            ranked: HashSet::new(),
            displayed: HashSet::new(),
            indexed: Vec::new(),
            indexed_map: HashMap::new(),
            index_new_fields: true,
        }
    }

    pub fn identifier(&self) -> String {
        self.fields_map.name(self.identifier).unwrap().to_string()
    }

    pub fn set_identifier(&mut self, id: String) -> SResult<()> {
        match self.id(id.clone()) {
            Some(id) => {
                self.identifier = id;
                Ok(())
            },
            None => Err(Error::FieldNameNotFound(id))
        }
    }

    pub fn id<S: Into<String>>(&self, name: S) -> Option<FieldId> {
        self.fields_map.id(name)
    }

    pub fn name<I: Into<FieldId>>(&self, id: I) -> Option<String> {
        self.fields_map.name(id)
    }

    pub fn contains<S: Into<String>>(&self, name: S) -> bool {
        self.fields_map.id(name.into()).is_some()
    }

    pub fn get_or_create_empty<S: Into<String>>(&mut self, name: S) -> SResult<FieldId> {
        self.fields_map.insert(name)
    }

    pub fn get_or_create<S: Into<String> + std::clone::Clone>(&mut self, name: S) -> SResult<FieldId> {
        match self.fields_map.id(name.clone()) {
            Some(id) => {
                Ok(id)
            }
            None => {
                if self.index_new_fields {
                    self.set_indexed(name.clone())?;
                    self.set_displayed(name)
                } else {
                    self.fields_map.insert(name.clone())
                }
            }
        }
    }

    pub fn ranked(&self) -> HashSet<FieldId> {
        self.ranked.clone()
    }

    pub fn ranked_name(&self) -> HashSet<String> {
        self.ranked.iter().filter_map(|a| self.name(*a)).collect()
    }

    pub fn displayed(&self) -> HashSet<FieldId> {
        self.displayed.clone()
    }

    pub fn displayed_name(&self) -> HashSet<String> {
        self.displayed.iter().filter_map(|a| self.name(*a)).collect()
    }

    pub fn indexed(&self) -> Vec<FieldId> {
        self.indexed.clone()
    }

    pub fn indexed_name(&self) -> Vec<String> {
        self.indexed.iter().filter_map(|a| self.name(*a)).collect()
    }

    pub fn set_ranked<S: Into<String>>(&mut self, name: S) -> SResult<FieldId> {
        let id = self.fields_map.insert(name.into())?;
        self.ranked.insert(id);
        Ok(id)
    }

    pub fn set_displayed<S: Into<String>>(&mut self, name: S) -> SResult<FieldId> {
        let id = self.fields_map.insert(name.into())?;
        self.displayed.insert(id);
        Ok(id)
    }

    pub fn set_indexed<S: Into<String>>(&mut self, name: S) -> SResult<(FieldId, IndexedPos)> {
        let id = self.fields_map.insert(name.into())?;
        if let Some(indexed_pos) = self.indexed_map.get(&id) {
            return Ok((id, *indexed_pos))
        };
        let pos = self.indexed.len() as u16;
        self.indexed.push(id);
        self.indexed_map.insert(id, pos.into());
        Ok((id, pos.into()))
    }

    pub fn remove_ranked<S: Into<String>>(&mut self, name: S) {
        if let Some(id) = self.fields_map.id(name.into()) {
            self.ranked.remove(&id);
        }
    }

    pub fn remove_displayed<S: Into<String>>(&mut self, name: S) {
        if let Some(id) = self.fields_map.id(name.into()) {
            self.displayed.remove(&id);
        }
    }

    pub fn remove_indexed<S: Into<String>>(&mut self, name: S) {
        if let Some(id) = self.fields_map.id(name.into()) {
            self.indexed_map.remove(&id);
            self.indexed.retain(|x| *x != id);
        }
    }

    pub fn is_ranked(&self, id: FieldId) -> bool {
        self.ranked.get(&id).is_some()
    }

    pub fn is_displayed(&self, id: FieldId) -> bool {
        self.displayed.get(&id).is_some()
    }

    pub fn is_indexed(&self, id: FieldId) -> Option<&IndexedPos> {
        self.indexed_map.get(&id)
    }

    pub fn indexed_pos_to_field_id<I: Into<IndexedPos>>(&self, pos: I) -> Option<FieldId> {
        let indexed_pos = pos.into().0 as usize;
        if indexed_pos < self.indexed.len() {
            Some(self.indexed[indexed_pos as usize])
        } else {
            None
        }
    }

    pub fn update_ranked<S: Into<String>>(&mut self, data: impl IntoIterator<Item = S>) -> SResult<()> {
        self.ranked = HashSet::new();
        for name in data {
            self.set_ranked(name)?;
        }
        Ok(())
    }

    pub fn update_displayed<S: Into<String>>(&mut self, data: impl IntoIterator<Item = S>) -> SResult<()> {
        self.displayed = HashSet::new();
        for name in data {
            self.set_displayed(name)?;
        }
        Ok(())
    }

    pub fn update_indexed<S: Into<String>>(&mut self, data: Vec<S>) -> SResult<()> {
        self.indexed = Vec::new();
        self.indexed_map = HashMap::new();
        for name in data {
            self.set_indexed(name)?;
        }
        Ok(())
    }

    pub fn index_new_fields(&self) -> bool {
        self.index_new_fields
    }

    pub fn set_index_new_fields(&mut self, value: bool) {
        self.index_new_fields = value;
    }
}
