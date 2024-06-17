use powdr_ast::analyzed::Analyzed;
use std::io;

use powdr_number::{Bn254Field, DegreeType, FieldElement};

use crate::vm_builder::analyzed_to_cpp;

// TODO: there will need to be multiple files that are generated, one for each relation

/// Barretenberg codegen
///
/// This module will take pil compiler output and make it generate relation header files that can be compiled into bberg

pub struct BBergCodegen {
    // Note: Im not sure we need to know the degree ahead of time
    // degree: DegreeType,
}

impl BBergCodegen {
    pub fn new(_degree: DegreeType) -> Self {
        Self {}
    }

    pub fn new_from_setup(_input: &mut impl io::Read) -> Result<Self, io::Error> {
        log::warn!("warning bberg: new_from_setup not implemented");
        Ok(Self {})
    }

    // Note: only returns vec<u8> to keep with the interface
    pub fn build_ast<F: FieldElement>(
        &self,
        pil: &Analyzed<F>,
        fixed: &[(String, Vec<F>)],
        witness: &[(String, Vec<F>)],
        bname: Option<String>,
    ) -> Vec<u8> {
        analyzed_to_cpp(pil, fixed, witness, bname);

        Vec::new()
    }

}
