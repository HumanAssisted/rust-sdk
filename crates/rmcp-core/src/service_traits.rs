use crate::{
    error::Error as McpError,
    model::{CancelledNotification, GetMeta, ProgressToken, RequestId, NumberOrString},
};
#[cfg(feature = "async_traits")]
use futures_core::future::BoxFuture;
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Debug, future::Future, sync::{atomic::AtomicU32, Arc}};

// --- TransferObject ---

pub trait TransferObject: Debug + Clone + Serialize + DeserializeOwned + Send + Sync + 'static {}

impl<T> TransferObject for T where
    T: Debug + Clone + Serialize + DeserializeOwned + Send + Sync + 'static
{
}

// --- ServiceRole ---

#[allow(private_bounds, reason = "there's no the third implementation")]
pub trait ServiceRole: Debug + Send + Sync + 'static + Copy + Clone {
    type Req: TransferObject + GetMeta;
    type Resp: TransferObject;
    type Not: TryInto<CancelledNotification, Error = Self::Not>
        + From<CancelledNotification>
        + TransferObject;
    type PeerReq: TransferObject + GetMeta;
    type PeerResp: TransferObject;
    type PeerNot: TryInto<CancelledNotification, Error = Self::PeerNot>
        + From<CancelledNotification>
        + TransferObject;
    const IS_CLIENT: bool;
    type Info: TransferObject;
    type PeerInfo: TransferObject;
}

// --- Message Type Aliases ---
// Note: JsonRpcMessage is defined in model, these rely on ServiceRole to specialize it.
// We might need to move these later if they cause circular dependencies or belong more
// closely with the runtime message handling. For now, let's keep them here as they
// are defined based on the core ServiceRole.
// pub type TxJsonRpcMessage<R> =
//     crate::model::JsonRpcMessage<<R as ServiceRole>::Req, <R as ServiceRole>::Resp, <R as ServiceRole>::Not>;
// pub type RxJsonRpcMessage<R> = crate::model::JsonRpcMessage<
//     <R as ServiceRole>::PeerReq,
//     <R as ServiceRole>::PeerResp,
//     <R as ServiceRole>::PeerNot,
// >;

// Forward declaration for RequestContext if needed by Service trait signature
// This might need adjustment depending on where RequestContext ends up.
// For now, assume it might be needed, but keep it minimal or generic if possible.
// If Service::handle_request doesn't *need* the concrete Peer details from the runtime crate,
// we can define a simpler version here. Let's omit it for now and see if Service compiles.

// --- Service Trait ---

// Forward declaration, replace with actual type if possible without runtime deps
pub struct RequestContext<R: ServiceRole> {
    // Placeholder - actual fields depend on runtime Peer/RequestHandle
     _marker: std::marker::PhantomData<R>,
     // Minimal context needed by core trait implementations?
     pub request_id: RequestId,
     // Potentially add CancellationToken reference if core needs cancellation awareness?
     // pub cancellation_token: Option<&tokio_util::sync::CancellationToken>, // No! Avoid tokio deps
}


/// Defines the core message handling logic for an MCP endpoint.
pub trait Service<R: ServiceRole>: Send + Sync + 'static {
    /// Handles an incoming request from the peer.
    fn handle_request(
        &self,
        request: R::PeerReq,
        context: RequestContext<R>, // Use placeholder context for now
    ) -> impl Future<Output = Result<R::Resp, McpError>> + Send + '_;

    /// Handles an incoming notification from the peer.
    fn handle_notification(
        &self,
        notification: R::PeerNot,
        // context: RequestContext<R>, // Add context if notifications need it
    ) -> impl Future<Output = Result<(), McpError>> + Send + '_;

    // Methods potentially tied to runtime Peer state.
    // Decide if these belong in the core trait or an extension trait.
    // Let's keep them for now, but implementations might need runtime details.
    // fn get_peer(&self) -> Option<Peer<R>>; // Peer is runtime-specific
    // fn set_peer(&mut self, peer: Peer<R>); // Peer is runtime-specific

    /// Gets the information/capabilities of this service endpoint.
    fn get_info(&self) -> R::Info;
}


// --- DynService Trait (Requires async_traits feature) ---

#[cfg(feature = "async_traits")]
pub trait DynService<R: ServiceRole>: Send + Sync {
    fn handle_request(
        &self,
        request: R::PeerReq,
        context: RequestContext<R>,
    ) -> BoxFuture<Result<R::Resp, McpError>>;

    fn handle_notification(
        &self,
        notification: R::PeerNot,
        // context: RequestContext<R>, // Add context if notifications need it
    ) -> BoxFuture<Result<(), McpError>>;

    // fn get_peer(&self) -> Option<Peer<R>>; // Runtime specific
    // fn set_peer(&mut self, peer: Peer<R>); // Runtime specific

    fn get_info(&self) -> R::Info;
}

#[cfg(feature = "async_traits")]
impl<R: ServiceRole, S: Service<R>> DynService<R> for S {
    fn handle_request(
        &self,
        request: R::PeerReq,
        context: RequestContext<R>,
    ) -> BoxFuture<Result<R::Resp, McpError>> {
        Box::pin(self.handle_request(request, context))
    }

    fn handle_notification(
        &self,
        notification: R::PeerNot,
        // context: RequestContext<R>,
    ) -> BoxFuture<Result<(), McpError>> {
        Box::pin(self.handle_notification(notification /*, context*/))
    }

    // fn get_peer(&self) -> Option<Peer<R>> {
    //     self.get_peer() // Delegates to Service trait, but Peer is runtime specific
    // }
    //
    // fn set_peer(&mut self, peer: Peer<R>) {
    //     self.set_peer(peer) // Delegates to Service trait, but Peer is runtime specific
    // }

    fn get_info(&self) -> R::Info {
        self.get_info()
    }
}


// --- ID Providers ---

pub trait RequestIdProvider: Send + Sync + 'static {
    fn next_request_id(&self) -> RequestId;
}

pub trait ProgressTokenProvider: Send + Sync + 'static {
    fn next_progress_token(&self) -> ProgressToken;
}

#[derive(Debug, Default)]
pub struct AtomicU32Provider {
    id: AtomicU32,
}

impl RequestIdProvider for AtomicU32Provider {
    fn next_request_id(&self) -> RequestId {
        // Use SeqCst for simplicity, could optimize ordering later if needed
        NumberOrString::Number(self.id.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
    }
}

impl ProgressTokenProvider for AtomicU32Provider {
    fn next_progress_token(&self) -> ProgressToken {
        ProgressToken(NumberOrString::Number(
            self.id.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        ))
    }
}

// Type aliases for convenience
pub type AtomicU32RequestIdProvider = AtomicU32Provider;
pub type AtomicU32ProgressTokenProvider = AtomicU32Provider; 