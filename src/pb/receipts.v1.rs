// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Receipts {
    #[prost(message, repeated, tag="1")]
    pub chunk_receipts: ::prost::alloc::vec::Vec<Receipt>,
    #[prost(message, repeated, tag="2")]
    pub outcome_receipts: ::prost::alloc::vec::Vec<Receipt>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Receipt {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
