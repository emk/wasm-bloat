use std::io::{Read, Write};

use index_map::*;
use parity_wasm::elements::{Deserialize, Error, Serialize, VarUint32, VarUint7};

const NAME_TYPE_MODULE: u8 = 0;
const NAME_TYPE_FUNCTION: u8 = 1;
const NAME_TYPE_LOCAL: u8 = 2;

/// Debug name information.
pub enum NameSection {
    /// Module name section.
    Module(ModuleNameSection),

    /// Function name section.
    Function(FunctionNameSection),

    /// Local name section.
    Local(LocalNameSection),

    /// Name section is unparsed.
    Unparsed {
        name_type: u8,
        name_payload: Vec<u8>,
    }
}

impl Serialize for NameSection {
    type Error = Error;

    fn serialize<W: Write>(self, wtr: &mut W) -> Result<(), Error> {
        unimplemented!()
    }
}

impl Deserialize for NameSection {
    type Error = Error;

    fn deserialize<R: Read>(rdr: &mut R) -> Result<NameSection, Error> {
        let name_type: u8 = VarUint7::deserialize(rdr)?.into();
        let name_payload_len: u32 = VarUint32::deserialize(rdr)?.into();
        let name_section = match name_type {
            NAME_TYPE_MODULE => {
                NameSection::Module(ModuleNameSection::deserialize(rdr)?)
            }
            NAME_TYPE_FUNCTION => {
                NameSection::Function(FunctionNameSection::deserialize(rdr)?)
            }
            NAME_TYPE_LOCAL => {
                NameSection::Local(LocalNameSection::deserialize(rdr)?)
            }
            _ => {
                // TODO: Read `name_payload_len` bytes.
                let name_payload = unimplemented!();
                NameSection::Unparsed { name_type, name_payload }
            }
        };
        Ok(name_section)
    }
}

/// The name of this module.
pub struct ModuleNameSection {
    name_str: String,
}

impl ModuleNameSection {
    /// The name of this module.
    pub fn name_str(&self) -> &str {
        &self.name_str
    }

    /// The name of this module (mutable).
    pub fn name_str_mut(&mut self) -> &mut String {
        &mut self.name_str
    }
}

impl Serialize for ModuleNameSection {
    type Error = Error;

    fn serialize<W: Write>(self, wtr: &mut W) -> Result<(), Error> {
        self.name_str.serialize(wtr)
    }
}

impl Deserialize for ModuleNameSection {
    type Error = Error;

    fn deserialize<R: Read>(rdr: &mut R) -> Result<ModuleNameSection, Error> {
        let name_str = String::deserialize(rdr)?;
        Ok(ModuleNameSection { name_str })
    }
}

/// The names of the functions in this module.
pub struct FunctionNameSection {
    names: NameMap,
}

impl FunctionNameSection {
    /// A map from function indices to names.
    pub fn names(&self) -> &NameMap {
        &self.names
    }

    /// A map from function indices to names (mutable).
    pub fn names_mut(&mut self) -> &mut NameMap {
        &mut self.names
    }
}

impl Serialize for FunctionNameSection {
    type Error = Error;

    fn serialize<W: Write>(self, wtr: &mut W) -> Result<(), Error> {
        self.names.serialize(wtr)
    }
}

impl Deserialize for FunctionNameSection {
    type Error = Error;

    fn deserialize<R: Read>(rdr: &mut R) -> Result<FunctionNameSection, Error> {
        let names = IndexMap::deserialize(rdr)?;
        Ok(FunctionNameSection { names })
    }
}

/// The names of the local variables in this module's functions.
pub struct LocalNameSection {
    local_names: IndexMap<NameMap>,
}

impl LocalNameSection {
    /// A map from function indices to a map from variables indices to names.
    pub fn local_names(&self) -> &IndexMap<NameMap> {
        &self.local_names
    }

    /// A map from function indices to a map from variables indices to names
    /// (mutable).
    pub fn local_names_mut(&mut self) -> &mut IndexMap<NameMap> {
        &mut self.local_names
    }
}

impl Serialize for LocalNameSection {
    type Error = Error;

    fn serialize<W: Write>(self, wtr: &mut W) -> Result<(), Error> {
        self.local_names.serialize(wtr)
    }
}

impl Deserialize for LocalNameSection {
    type Error = Error;

    fn deserialize<R: Read>(rdr: &mut R) -> Result<LocalNameSection, Error> {
        let local_names = IndexMap::deserialize(rdr)?;
        Ok(LocalNameSection { local_names })
    }
}

/// A map from indices to names.
pub type NameMap = IndexMap<String>;
