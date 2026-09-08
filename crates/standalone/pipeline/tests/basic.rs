use quantaradar_pipeline::*;
#[test]
fn pipeline_start_stop() {
    let mut p = Pipeline::new();
    assert!(p.start());
    assert!(p.stop());
}
