#[derive(Clone, Debug, cdrs_tokio_helpers_derive::IntoCDRSValue, cdrs_tokio_helpers_derive::TryFromUDT)]
struct N {
    pub n: i16,
}
