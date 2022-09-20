# rebus-grpc-client

This is a GRPC client package of [rebuschain/rebus.core](https://github.com/rebuschain/rebus.core) for Rust.

**Depended libraries**

- [tonic](https://github.com/hyperium/tonic)
- [tokio 1](https://github.com/tokio-rs/tokio)

**proto files**

- see [rebus.core/go.mod at v0.1.2](https://github.com/rebuschain/rebus.core/blob/v0.1.2/go.mod) for listing related
packages versions.
- and also [cosmos-sdk/go.mod at v0.45.5](https://github.com/cosmos/cosmos-sdk/blob/v0.45.5/go.mod)

---

- [rebus.core/proto/rebus at v0.1.2](https://github.com/rebuschain/rebus.core/tree/v0.1.2/proto/rebus)
- [cosmos-sdk/proto at v0.45.5](https://github.com/cosmos/cosmos-sdk/tree/v0.45.5/proto)
- [ibc-go/proto at v3.1.0](https://github.com/cosmos/ibc-go/tree/v3.1.0/proto)
- [tendermint/proto/tendermint at v0.34.19](https://github.com/tendermint/tendermint/tree/v0.34.19/proto/tendermint)
- [cosmos/cosmos-proto](https://github.com/cosmos/cosmos-proto/tree/main/proto/cosmos_proto)
- [ics23/proofs.proto at v0.6.6](https://github.com/confio/ics23/blob/v0.6.6/proofs.proto)
- [protobuf/gogo.proto at v1.3.3-alpha.regen.1](https://github.com/regen-network/protobuf/blob/v1.3.3-alpha.regen.1/gogoproto/gogo.proto)
- [google/api](https://fuchsia.googlesource.com/third_party/googleapis)

```bash
% find src/proto/cosmos -type f -follow -print | awk '{print "\""$0"\","}'
% find src/proto/ibc -type f -follow -print | awk '{print "\""$0"\","}'
% find src/proto/rebus -type f -follow -print | awk '{print "\""$0"\","}'
% find src/proto/evmos -type f -follow -print | awk '{print "\""$0"\","}'
```

## Installation

#### Dependencies

- [Rust with Cargo](http://rust-lang.org)
- [protoc](https://grpc.io/docs/protoc-installation/)

#### Importing

**Cargo.toml**

```toml
[dependencies]
rebus-grpc-client = { version = "0.1.2", git = "https://github.com/kumanote/rebus-grpc-client-rs", branch = "main" }
```

**rust files**

```rust
use rebus_grpc_client::{cosmos, tonic};
```

## Usage

Here's a basic example:

```rust
use rebus_grpc_client::{cosmos, tonic};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "http://localhost:9090";
    let mut client =
        cosmos::base::tendermint::v1beta1::service_client::ServiceClient::connect(addr).await?;
    let request = tonic::Request::new(cosmos::base::tendermint::v1beta1::GetLatestBlockRequest {});
    let response = client.get_latest_block(request).await?;
    let latest_height = response.into_inner().block.unwrap().header.unwrap().height;
    let request = tonic::Request::new(cosmos::base::tendermint::v1beta1::GetBlockByHeightRequest {
        height: latest_height,
    });
    let response = client.get_block_by_height(request).await?;
    assert_eq!(
        latest_height,
        response.into_inner().block.unwrap().header.unwrap().height
    );
    Ok(())
}
```
