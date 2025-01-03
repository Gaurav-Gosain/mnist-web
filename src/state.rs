use crate::model::Model;
use burn::{
    module::Module,
    record::{BinBytesRecorder, FullPrecisionSettings, Recorder},
};

pub type Backend = burn::backend::ndarray::NdArray<f32>;

static STATE_ENCODED: &[u8] = include_bytes!("../model.bin");

/// Builds and loads trained parameters into the model.
pub async fn build_and_load_model() -> Model<Backend> {
    let model: Model<Backend> = Model::new(&Default::default());
    let record = BinBytesRecorder::<FullPrecisionSettings>::default()
        .load(STATE_ENCODED.to_vec(), &Default::default())
        .expect("Failed to decode state");

    model.load_record(record)
}
