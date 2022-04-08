/// Traits and types for all system components.
pub mod components;

/// Common data types used throughout The Graph.
pub mod data;

/// Utilities.
pub mod util;

/// Extension traits for external types.
pub mod ext;

/// Wrapper for spawning tasks that abort on panic, which is our default.
mod task_spawn;
pub use task_spawn::{
    block_on, spawn, spawn_allow_panic, spawn_blocking, spawn_blocking_allow_panic, spawn_thread,
};

pub use anyhow;
pub use bytes;
pub use parking_lot;
pub use petgraph;
pub use prometheus;
pub use semver;
pub use slog;
pub use stable_hash;
pub use tokio_stream;
pub use url;

/// A prelude that makes all system component traits and data types available.
///
/// Add the following code to import all traits and data types listed below at once.
///
/// ```
/// use graph::prelude::*;
/// ```
pub mod prelude {
    pub use super::entity;
    pub use ::anyhow;
    pub use anyhow::{anyhow, Context as _, Error};
    pub use async_trait::async_trait;
    pub use bigdecimal;
    pub use chrono;
    pub use envconfig;
    pub use ethabi;
    pub use futures::future;
    pub use futures::prelude::*;
    pub use futures::stream;
    pub use futures03;
    pub use futures03::compat::{Future01CompatExt, Sink01CompatExt, Stream01CompatExt};
    pub use futures03::future::{FutureExt as _, TryFutureExt};
    pub use futures03::sink::SinkExt as _;
    pub use futures03::stream::{StreamExt as _, TryStreamExt};
    pub use hex;
    pub use lazy_static::lazy_static;
    pub use prost;
    pub use rand;
    pub use reqwest;
    pub use serde;
    pub use serde_derive::{Deserialize, Serialize};
    pub use serde_json;
    pub use serde_yaml;
    pub use slog::{self, crit, debug, error, info, o, trace, warn, Logger};
    pub use std::convert::TryFrom;
    pub use std::fmt::Debug;
    pub use std::iter::FromIterator;
    pub use std::pin::Pin;
    pub use std::sync::Arc;
    pub use std::time::Duration;
    pub use thiserror;
    pub use tiny_keccak;
    pub use tokio;
    pub use tonic;
    pub use web3;

    pub type DynTryFuture<'a, Ok = (), Err = Error> =
        Pin<Box<dyn futures03::Future<Output = Result<Ok, Err>> + Send + 'a>>;

    pub type BlockPtr = ();

    pub use crate::components::graphql::{
        GraphQlRunner, QueryLoadManager, SubscriptionResultFuture,
    };
    pub use crate::components::link_resolver::{JsonStreamValue, JsonValueStream, LinkResolver};
    pub use crate::components::store::{
        AttributeNames, BlockNumber, CachedEthereumCall, ChainStore, ChildMultiplicity,
        EntityCache, EntityChange, EntityChangeOperation, EntityCollection, EntityFilter,
        EntityKey, EntityLink, EntityModification, EntityOperation, EntityOrder, EntityQuery,
        EntityRange, EntityWindow, EthereumCallCache, ParentLink, PoolWaitStats, QueryStore,
        QueryStoreManager, StoreError, StoreEvent, StoreEventStream, StoreEventStreamBox,
        SubgraphStore, UnfailOutcome, WindowAttribute, BLOCK_NUMBER_MAX,
    };
    pub use crate::components::{transaction_receipt, EventConsumer, EventProducer};

    pub use crate::data::graphql::{shape_hash::shape_hash, SerializableValue, TryFromValue};
    pub use crate::data::query::{
        Query, QueryError, QueryExecutionError, QueryResult, QueryVariables,
    };
    pub use crate::data::schema::{ApiSchema, Schema};
    pub use crate::data::store::ethereum::*;
    pub use crate::data::store::scalar::{BigDecimal, BigInt, BigIntSign};
    pub use crate::data::store::{
        AssignmentEvent, Attribute, Entity, NodeId, SubscriptionFilter, ToEntityId, ToEntityKey,
        TryIntoEntity, Value, ValueType,
    };
    pub use crate::data::subgraph::schema::SubgraphDeploymentEntity;
    pub use crate::data::subgraph::{
        CreateSubgraphResult, DataSourceContext, DeploymentHash, DeploymentState, Link,
        SubgraphAssignmentProviderError, SubgraphManifest, SubgraphManifestResolveError,
        SubgraphManifestValidationError, SubgraphName, SubgraphRegistrarError,
        UnvalidatedSubgraphManifest,
    };
    pub use crate::data::subscription::{
        QueryResultStream, Subscription, SubscriptionError, SubscriptionResult,
    };
    pub use crate::ext::futures::{
        CancelGuard, CancelHandle, CancelToken, CancelableError, FutureExtension,
        SharedCancelGuard, StreamExtension,
    };
    pub use crate::util::cache_weight::CacheWeight;
    pub use crate::util::futures::{retry, TimeoutError};
    pub use crate::util::stats::MovingStats;

    macro_rules! static_graphql {
        ($m:ident, $m2:ident, {$($n:ident,)*}) => {
            pub mod $m {
                use graphql_parser::$m2 as $m;
                pub use $m::*;
                $(
                    pub type $n = $m::$n<'static, String>;
                )*
            }
        };
    }

    // Static graphql mods. These are to be phased out, with a preference
    // toward making graphql generic over text. This helps to ease the
    // transition by providing the old graphql-parse 0.2.x API
    static_graphql!(q, query, {
        Document, Value, OperationDefinition, InlineFragment, TypeCondition,
        FragmentSpread, Field, Selection, SelectionSet, FragmentDefinition,
        Directive, VariableDefinition, Type, Query,
    });
    static_graphql!(s, schema, {
        Field, Directive, InterfaceType, ObjectType, Value, TypeDefinition,
        EnumType, Type, Document, ScalarType, InputValue, DirectiveDefinition,
        UnionType, InputObjectType, EnumValue,
    });

    pub mod r {
        pub use crate::data::value::Value;
    }
}
