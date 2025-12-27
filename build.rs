fn main() {
    let protos = [
        // common protos
        "proto/spire/api/types/agent.proto",
        "proto/spire/api/types/attestation.proto",
        "proto/spire/api/types/bundle.proto",
        "proto/spire/api/types/entry.proto",
        "proto/spire/api/types/federateswith.proto",
        "proto/spire/api/types/federationrelationship.proto",
        "proto/spire/api/types/jointoken.proto",
        "proto/spire/api/types/jwtsvid.proto",
        "proto/spire/api/types/logger.proto",
        "proto/spire/api/types/selector.proto",
        "proto/spire/api/types/spiffeid.proto",
        "proto/spire/api/types/status.proto",
        "proto/spire/api/types/witsvid.proto",
        "proto/spire/api/types/x509svid.proto",
        // agent protos
        #[cfg(feature = "agent-debug")]
        "proto/spire/api/agent/debug/v1/debug.proto",
        #[cfg(feature = "agent-delegatedidentity")]
        "proto/spire/api/agent/delegatedidentity/v1/delegatedidentity.proto",
        #[cfg(feature = "agent-logger")]
        "proto/spire/api/agent/logger/v1/logger.proto",
        // server protos
        #[cfg(feature = "server-agent")]
        "proto/spire/api/server/agent/v1/agent.proto",
        #[cfg(feature = "server-bundle")]
        "proto/spire/api/server/bundle/v1/bundle.proto",
        #[cfg(feature = "server-debug")]
        "proto/spire/api/server/debug/v1/debug.proto",
        #[cfg(feature = "server-entry")]
        "proto/spire/api/server/entry/v1/entry.proto",
        #[cfg(feature = "server-localauthority")]
        "proto/spire/api/server/localauthority/v1/localauthority.proto",
        #[cfg(feature = "server-logger")]
        "proto/spire/api/server/logger/v1/logger.proto",
        #[cfg(feature = "server-svid")]
        "proto/spire/api/server/svid/v1/svid.proto",
        #[cfg(feature = "server-trustdomain")]
        "proto/spire/api/server/trustdomain/v1/trustdomain.proto",
    ];

    tonic_prost_build::configure()
        .include_file("spire_api.rs")
        // .bytes(None)
        .build_transport(false)
        .emit_rerun_if_changed(true)
        .compile_protos(&protos, &["proto"])
        .unwrap();
}
