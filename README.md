<p align="center">
  <img width="945" height="432" src="./logo.png">
    <a href="https://github.com/BudiNverse/justbus-rs">
      <img src="https://img.shields.io/badge/-justbus--rs-blueviolet.svg"/>
    </a>
    <a href="https://github.com/BudiNverse/justbus-rs">
        <img src="https://img.shields.io/github/license/BudiNverse/lta-rs"/>
    </a>
    <a href="https://github.com/BudiNverse/lta-rs">
        <img src="https://img.shields.io/badge/rust-1.3.9-blueviolet.svg"/>
    </a>      
</p>

# `justbus-grpc`
> A lightweight gRPC implementation of [justbus-rs](https://github.com/BudiNverse/justbus-rs)

## Usage
```
grpcurl -plaintext -import-path ./proto -proto buses.proto -d '{"bus_stop_id": 83139}' [::]:50051 buses.JustBus/Timings
```
