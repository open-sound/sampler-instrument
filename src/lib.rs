/// A standard generic interface for sampler instruments.
#[derive(Debug)]
pub struct SamplerInstrument {
    name: String,
    groups: Vec<Group>,
}

#[derive(Debug)]
pub struct Group {
    name: String,
    zones: Vec<Zone>,
    parameters: Vec<Parameter>,
}

#[derive(Debug)]
pub struct Zone {
    sample_path: String,
    root_key: u8,
    low_key: u8,
    high_key: u8,
    parameters: Vec<Parameter>,
}

#[derive(Debug)]
pub struct Parameter {
    name: String,
    value: f64,
}
