use rusmpp::extra::encoding::{
    gsm7bit::{
        Gsm7BitAlphabet as RGsm7BitAlphabet, Gsm7BitDefaultAlphabet as RGsm7BitDefaultAlphabet,
        Gsm7BitUnpackedEncoder as RGsm7BitUnpacked,
    },
    latin1::Latin1Encoder as RLatin1,
    ucs2::Ucs2Encoder as RUcs2,
};

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum Encoder {
    Gsm7BitUnpacked(Gsm7BitUnpacked),
    Ucs2(Ucs2),
    Latin1(Latin1),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum Gsm7BitAlphabet {
    Default(Gsm7BitDefaultAlphabet),
}

impl Gsm7BitAlphabet {
    pub fn default_() -> Self {
        Self::Default(Gsm7BitDefaultAlphabet {})
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pymethods]
#[::pyo3::pymethods]
impl Gsm7BitAlphabet {
    #[classmethod]
    #[pyo3(signature=())]
    pub fn default<'p>(_cls: &'p ::pyo3::Bound<'p, ::pyo3::types::PyType>) -> Self {
        Self::default_()
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct Gsm7BitDefaultAlphabet {}

#[::pyo3_stub_gen_derive::gen_stub_pymethods]
#[::pyo3::pymethods]
impl Gsm7BitDefaultAlphabet {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct Gsm7BitUnpacked {
    alphabet: Gsm7BitAlphabet,
    allow_split_extended_character: bool,
}

impl Gsm7BitUnpacked {
    pub fn default_() -> Self {
        Self {
            alphabet: Gsm7BitAlphabet::Default(Gsm7BitDefaultAlphabet {}),
            allow_split_extended_character: false,
        }
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pymethods]
#[::pyo3::pymethods]
impl Gsm7BitUnpacked {
    #[new]
    #[pyo3(signature=(alphabet=Gsm7BitAlphabet::default_(), allow_split_extended_character=false, ))]
    fn new(alphabet: Gsm7BitAlphabet, allow_split_extended_character: bool) -> Self {
        Self {
            alphabet,
            allow_split_extended_character,
        }
    }

    #[classmethod]
    #[pyo3(signature=())]
    pub fn default<'p>(_cls: &'p ::pyo3::Bound<'p, ::pyo3::types::PyType>) -> Self {
        Self::default_()
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct Ucs2 {
    allow_split_character: bool,
}

impl Ucs2 {
    pub fn default_() -> Self {
        Self {
            allow_split_character: false,
        }
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pymethods]
#[::pyo3::pymethods]
impl Ucs2 {
    #[new]
    #[pyo3(signature=(allow_split_character=false))]
    fn new(allow_split_character: bool) -> Self {
        Self {
            allow_split_character,
        }
    }

    #[classmethod]
    #[pyo3(signature=())]
    pub fn default<'p>(_cls: &'p ::pyo3::Bound<'p, ::pyo3::types::PyType>) -> Self {
        Self::default_()
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct Latin1 {}

impl Latin1 {
    pub fn default_() -> Self {
        Self {}
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pymethods]
#[::pyo3::pymethods]
impl Latin1 {
    #[new]

    fn new() -> Self {
        Self {}
    }

    #[classmethod]
    #[pyo3(signature=())]
    pub fn default<'p>(_cls: &'p ::pyo3::Bound<'p, ::pyo3::types::PyType>) -> Self {
        Self::default_()
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

impl From<Gsm7BitAlphabet> for RGsm7BitAlphabet {
    fn from(value: Gsm7BitAlphabet) -> Self {
        match value {
            Gsm7BitAlphabet::Default(_) => {
                RGsm7BitAlphabet::Default(RGsm7BitDefaultAlphabet::new())
            }
        }
    }
}

impl From<Gsm7BitUnpacked> for RGsm7BitUnpacked {
    fn from(value: Gsm7BitUnpacked) -> Self {
        Self::new()
            .with_alphabet(value.alphabet.into())
            .with_allow_split_extended_character(value.allow_split_extended_character)
    }
}

impl From<Ucs2> for RUcs2 {
    fn from(value: Ucs2) -> Self {
        Self::new().with_allow_split_character(value.allow_split_character)
    }
}

impl From<Latin1> for RLatin1 {
    fn from(_value: Latin1) -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct EncodeError {
    pub encoder: String,
    pub error: String,
}

impl std::fmt::Display for EncodeError {
    // XXX: do not add a prefix like `Encode error: ...`, otherwise we get `Encode error: Encode error: ...` in the final error message in the exception.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "encoder: {}, error: {}", self.encoder, self.error)
    }
}

impl EncodeError {
    pub const fn new(encoder: String, error: String) -> Self {
        Self { encoder, error }
    }
}

impl rusmpp::extra::encoding::Encoder for Encoder {
    type Error = EncodeError;

    fn encode(&self, message: &str) -> Result<(Vec<u8>, rusmpp::values::DataCoding), Self::Error> {
        match self {
            Encoder::Gsm7BitUnpacked(encoder) => RGsm7BitUnpacked::from(*encoder)
                .encode(message)
                .map_err(|e| {
                    EncodeError::new(String::from(stringify!(Gsm7BitUnpacked)), e.to_string())
                }),
            Encoder::Ucs2(encoder) => RUcs2::from(*encoder)
                .encode(message)
                .map_err(|e| EncodeError::new(String::from(stringify!(Ucs2)), e.to_string())),
            Encoder::Latin1(encoder) => RLatin1::from(*encoder)
                .encode(message)
                .map_err(|e| EncodeError::new(String::from(stringify!(Latin1)), e.to_string())),
        }
    }
}

impl rusmpp::extra::concatenation::Concatenator for Encoder {
    type Error = EncodeError;

    fn concatenate(
        &self,
        message: &str,
        max_message_size: usize,
        part_header_size: usize,
    ) -> Result<
        (
            rusmpp::extra::concatenation::Concatenation,
            rusmpp::values::DataCoding,
        ),
        Self::Error,
    > {
        match self {
            Encoder::Gsm7BitUnpacked(encoder) => RGsm7BitUnpacked::from(*encoder)
                .concatenate(message, max_message_size, part_header_size)
                .map_err(|e| {
                    EncodeError::new(String::from(stringify!(Gsm7BitUnpacked)), e.to_string())
                }),
            Encoder::Ucs2(encoder) => RUcs2::from(*encoder)
                .concatenate(message, max_message_size, part_header_size)
                .map_err(|e| EncodeError::new(String::from(stringify!(Ucs2)), e.to_string())),
            Encoder::Latin1(encoder) => RLatin1::from(*encoder)
                .concatenate(message, max_message_size, part_header_size)
                .map_err(|e| EncodeError::new(String::from(stringify!(Latin1)), e.to_string())),
        }
    }
}
