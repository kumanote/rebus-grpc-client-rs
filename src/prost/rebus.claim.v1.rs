/// A Claim Records is the metadata of claim data per address
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimRecord {
    /// address of claim user
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// total initial claimable amount for the user
    #[prost(message, repeated, tag = "2")]
    pub initial_claimable_amount:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// true if action is completed
    /// index of bool in array refers to action enum #
    #[prost(bool, repeated, packed = "false", tag = "3")]
    pub action_completed: ::prost::alloc::vec::Vec<bool>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Action {
    Delegate = 0,
    Vote = 1,
    NftId = 2,
    Vault = 3,
}
/// Params defines the claim module's parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(message, optional, tag = "1")]
    pub airdrop_start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "2")]
    pub airdrop_duration: ::core::option::Option<::prost_types::Duration>,
    /// denom of claimable asset
    #[prost(string, tag = "3")]
    pub claim_denom: ::prost::alloc::string::String,
    /// claim flag enable/disable
    #[prost(bool, tag = "4")]
    pub claim_enabled: bool,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleAccountBalanceRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleAccountBalanceResponse {
    /// params defines the parameters of the module.
    #[prost(message, repeated, tag = "1")]
    pub module_account_balance:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClaimRecordRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClaimRecordResponse {
    #[prost(message, optional, tag = "1")]
    pub claim_record: ::core::option::Option<ClaimRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClaimableForActionRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(enumeration = "Action", tag = "2")]
    pub action: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClaimableForActionResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalClaimableRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalClaimableResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[doc = r" Generated client implementations."]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Query defines the gRPC querier service."]
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn module_account_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryModuleAccountBalanceRequest>,
        ) -> Result<tonic::Response<super::QueryModuleAccountBalanceResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/rebus.claim.v1.Query/ModuleAccountBalance");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/rebus.claim.v1.Query/Params");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn claim_record(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryClaimRecordRequest>,
        ) -> Result<tonic::Response<super::QueryClaimRecordResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/rebus.claim.v1.Query/ClaimRecord");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn claimable_for_action(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryClaimableForActionRequest>,
        ) -> Result<tonic::Response<super::QueryClaimableForActionResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/rebus.claim.v1.Query/ClaimableForAction");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn total_claimable(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTotalClaimableRequest>,
        ) -> Result<tonic::Response<super::QueryTotalClaimableResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/rebus.claim.v1.Query/TotalClaimable");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// GenesisState defines the claim module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// list of claim records, one for every airdrop recipient
    #[prost(message, repeated, tag = "2")]
    pub claim_records: ::prost::alloc::vec::Vec<ClaimRecord>,
}
