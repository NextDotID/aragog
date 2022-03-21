#![allow(clippy::too_many_lines)]
use thiserror::Error;

/// Internal Aragog error based on `ArangoDB` error num
#[derive(Debug, Copy, Clone, Error, PartialEq)]
pub enum ArangoError {
    /// 1000 - ERROR_ARANGO_ILLEGAL_STATE
    ///
    /// Internal error that will be raised when the datafile is not in the required state.
    #[error("1000 - ERROR_ARANGO_ILLEGAL_STATE")]
    ArangoIllegalState,
    /// 1002 - ERROR_ARANGO_DATAFILE_SEALED
    ///
    /// Internal error that will be raised when trying to write to a datafile.
    #[error("1002 - ERROR_ARANGO_DATAFILE_SEALED")]
    ArangoDatafileSealed,
    /// 1004 - ERROR_ARANGO_READ_ONLY
    ///
    /// Internal error that will be raised when trying to write to a read-only datafile or collection.
    #[error("1004 - ERROR_ARANGO_READ_ONLY")]
    ArangoReadOnly,
    /// 1005 - ERROR_ARANGO_DUPLICATE_IDENTIFIER
    ///
    /// Internal error that will be raised when a identifier duplicate is detected.
    #[error("1005 - ERROR_ARANGO_DUPLICATE_IDENTIFIER")]
    ArangoDuplicateIdentifier,
    /// 1006 - ERROR_ARANGO_DATAFILE_UNREADABLE
    ///
    /// Internal error that will be raised when a datafile is unreadable.
    #[error("1006 - ERROR_ARANGO_DATAFILE_UNREADABLE")]
    ArangoDatafileUnreadable,
    /// 1007 - ERROR_ARANGO_DATAFILE_EMPTY
    ///
    /// Internal error that will be raised when a datafile is empty.
    #[error("1007 - ERROR_ARANGO_DATAFILE_EMPTY")]
    ArangoDatafileEmpty,
    /// 1008 - ERROR_ARANGO_RECOVERY
    ///
    /// Will be raised when an error occurred during WAL log file recovery.
    #[error("1008 - ERROR_ARANGO_RECOVERY")]
    ArangoRecovery,
    /// 1009 - ERROR_ARANGO_DATAFILE_STATISTICS_NOT_FOUND
    ///
    /// Will be raised when a required datafile statistics object was not found.
    #[error("1009 - ERROR_ARANGO_DATAFILE_STATISTICS_NOT_FOUND")]
    ArangoDatafileStatisticsNotFound,
    /// 1100 - ERROR_ARANGO_CORRUPTED_DATAFILE
    ///
    /// Will be raised when a corruption is detected in a datafile.
    #[error("1100 - ERROR_ARANGO_CORRUPTED_DATAFILE")]
    ArangoCorruptedDatafile,
    /// 1101 - ERROR_ARANGO_ILLEGAL_PARAMETER_FILE
    ///
    /// Will be raised if a parameter file is corrupted or cannot be read.
    #[error("1101 - ERROR_ARANGO_ILLEGAL_PARAMETER_FILE")]
    ArangoIllegalParameterFile,
    /// 1102 - ERROR_ARANGO_CORRUPTED_COLLECTION
    ///
    /// Will be raised when a collection contains one or more corrupted data files.
    #[error("1102 - ERROR_ARANGO_CORRUPTED_COLLECTION")]
    ArangoCorruptedCollection,
    /// 1103 - ERROR_ARANGO_MMAP_FAILED
    ///
    /// Will be raised when the system call mmap failed.
    #[error("1103 - ERROR_ARANGO_MMAP_FAILED")]
    ArangoMmapFailed,
    /// 1104 - ERROR_ARANGO_FILESYSTEM_FULL
    ///
    /// Will be raised when the filesystem is full.
    #[error("1104 - ERROR_ARANGO_FILESYSTEM_FULL")]
    ArangoFileSystemFull,
    /// 1105 - ERROR_ARANGO_NO_JOURNAL
    ///
    /// Will be raised when a journal cannot be created.
    #[error("1105 - ERROR_ARANGO_NO_JOURNAL")]
    ArangoNoJournal,
    /// 1106 - ERROR_ARANGO_DATAFILE_ALREADY_EXISTS
    ///
    /// Will be raised when the datafile cannot be created or renamed because a file of the same name already exists.
    #[error("1106 - ERROR_ARANGO_DATAFILE_ALREADY_EXISTS")]
    ArangoDatafileAlreadyExists,
    /// 1107 - ERROR_ARANGO_DATADIR_LOCKED
    ///
    /// Will be raised when the database directory is locked by a different process.
    #[error("1107 - ERROR_ARANGO_DATADIR_LOCKED")]
    ArangoDatadirLocked,
    /// 1108 - ERROR_ARANGO_COLLECTION_DIRECTORY_ALREADY_EXISTS
    ///
    /// Will be raised when the collection cannot be created because a directory of the same name already exists.
    #[error("1108 - ERROR_ARANGO_COLLECTION_DIRECTORY_ALREADY_EXISTS")]
    ArangoCollectionDirectoryAlreadyExists,
    /// 1109 - ERROR_ARANGO_MSYNC_FAILED
    ///
    /// Will be raised when the system call msync failed.
    #[error("1109 - ERROR_ARANGO_MSYNC_FAILED")]
    ArangoMSyncFailed,
    /// 1110 - ERROR_ARANGO_DATADIR_UNLOCKABLE
    ///
    /// Will be raised when the server cannot lock the database directory on startup.
    #[error("1110 - ERROR_ARANGO_DATADIR_UNLOCKABLE")]
    ArangoDatadirUnlockable,
    /// 1111 - ERROR_ARANGO_SYNC_TIMEOUT
    ///
    /// Will be raised when the server waited too long for a datafile to be synced to disk.
    #[error("1111 - ERROR_ARANGO_SYNC_TIMEOUT")]
    ArangoSyncTimeout,
    /// 1200 - ERROR_ARANGO_CONFLICT
    ///
    /// Will be raised when updating or deleting a document and a conflict has been detected.
    #[error("1200 - ERROR_ARANGO_CONFLICT")]
    ArangoConflict,
    /// 1201 - ERROR_ARANGO_DATADIR_INVALID
    ///
    /// Will be raised when a non-existing database directory was specified when starting the database.
    #[error("1201 - ERROR_ARANGO_DATADIR_INVALID")]
    ArangoDatadirInvalid,
    /// 1202 - ERROR_ARANGO_DOCUMENT_NOT_FOUND
    ///
    /// Will be raised when a document with a given identifier is unknown.
    #[error("1202 - ERROR_ARANGO_DOCUMENT_NOT_FOUND")]
    ArangoDocumentNotFound,
    /// 1203 - ERROR_ARANGO_DATA_SOURCE_NOT_FOUND
    ///
    /// Will be raised when a collection or View with the given identifier or name is unknown.
    #[error("1203 - ERROR_ARANGO_DATA_SOURCE_NOT_FOUND")]
    ArangoDataSourceNotFound,
    /// 1204 - ERROR_ARANGO_COLLECTION_PARAMETER_MISSING
    ///
    /// Will be raised when the collection parameter is missing.
    #[error("1204 - ERROR_ARANGO_COLLECTION_PARAMETER_MISSING")]
    ArangoCollectionParameterMissing,
    /// 1205 - ERROR_ARANGO_DOCUMENT_HANDLE_BAD
    ///
    /// Will be raised when a document identifier is corrupt.
    #[error("1205 - ERROR_ARANGO_DOCUMENT_HANDLE_BAD")]
    ArangoDocumentHandleBad,
    /// 1206 - ERROR_ARANGO_MAXIMAL_SIZE_TOO_SMALL
    ///
    /// Will be raised when the maximal size of the journal is too small.
    #[error("1206 - ERROR_ARANGO_MAXIMAL_SIZE_TOO_SMALL")]
    ArangoMaximalSizeTooSmall,
    /// 1207 - ERROR_ARANGO_DUPLICATE_NAME
    ///
    /// Will be raised when a name duplicate is detected.
    #[error("1207 - ERROR_ARANGO_DUPLICATE_NAME")]
    ArangoDuplicateName,
    /// 1208 - ERROR_ARANGO_ILLEGAL_NAME
    ///
    /// Will be raised when an illegal name is detected.
    #[error("1208 - ERROR_ARANGO_ILLEGAL_NAME")]
    ArangoIllegalName,
    /// 1209 - ERROR_ARANGO_NO_INDEX
    ///
    /// Will be raised when no suitable index for the query is known.
    #[error("1209 - ERROR_ARANGO_NO_INDEX")]
    ArangoNoIndex,
    /// 1210 - ERROR_ARANGO_UNIQUE_CONSTRAINT_VIOLATED
    ///
    /// Will be raised when there is a unique constraint violation.
    #[error("1210 - ERROR_ARANGO_UNIQUE_CONSTRAINT_VIOLATED")]
    ArangoUniqueConstraintViolated,
    /// 1212 - ERROR_ARANGO_INDEX_NOT_FOUND
    ///
    /// Will be raised when an index with a given identifier is unknown.
    #[error("1212 - ERROR_ARANGO_INDEX_NOT_FOUND")]
    ArangoIndexNotFound,
    /// 1213 - ERROR_ARANGO_CROSS_COLLECTION_REQUEST
    ///
    /// Will be raised when a cross-collection is requested.
    #[error("1213 - ERROR_ARANGO_CROSS_COLLECTION_REQUEST")]
    ArangoCrossCollectionRequest,
    /// 1214 - ERROR_ARANGO_INDEX_HANDLE_BAD
    ///
    /// Will be raised when a index identifier is corrupt.
    #[error("1214 - ERROR_ARANGO_INDEX_HANDLE_BAD")]
    ArangoIndexHandleBad,
    /// 1216 - ERROR_ARANGO_DOCUMENT_TOO_LARGE
    ///
    /// Will be raised when the document cannot fit into any datafile because of it is too large.
    #[error("1216 - ERROR_ARANGO_DOCUMENT_TOO_LARGE")]
    ArangoDocumentTooLarge,
    /// 1217 - ERROR_ARANGO_COLLECTION_NOT_UNLOADED
    ///
    /// Will be raised when a collection should be unloaded
    #[error("1217 - ERROR_ARANGO_COLLECTION_NOT_UNLOADED")]
    ArangoCollectionNotUnloaded,
    /// 1218 - ERROR_ARANGO_COLLECTION_TYPE_INVALID
    ///
    /// Will be raised when an invalid collection type is used in a request.
    #[error("1218 - ERROR_ARANGO_COLLECTION_TYPE_INVALID")]
    ArangoCollectionTypeInvalid,
    /// 1220 - ERROR_ARANGO_ATTRIBUTE_PARSER_FAILED
    ///
    /// Will be raised when parsing an attribute name definition failed.
    #[error("1220 - ERROR_ARANGO_ATTRIBUTE_PARSER_FAILED")]
    ArangoAttributeParserFailed,
    /// 1221 - ERROR_ARANGO_DOCUMENT_KEY_BAD
    ///
    /// Will be raised when a document key is corrupt.
    #[error("1221 - ERROR_ARANGO_DOCUMENT_KEY_BAD")]
    ArangoDocumentKeyBad,
    /// 1222 - ERROR_ARANGO_DOCUMENT_KEY_UNEXPECTED
    ///
    /// Will be raised when a user-defined document key is supplied for collections with auto key generation.
    #[error("1222 - ERROR_ARANGO_DOCUMENT_KEY_UNEXPECTED")]
    ArangoDocumentKeyUnexpected,
    /// 1224 - ERROR_ARANGO_DATADIR_NOT_WRITABLE
    ///
    /// Will be raised when the server’s database directory is not writable for the current user.
    #[error("1224 - ERROR_ARANGO_DATADIR_NOT_WRITABLE")]
    ArangoDatadirNotWritable,
    /// 1225 - ERROR_ARANGO_OUT_OF_KEYS
    ///
    /// Will be raised when a key generator runs out of keys.
    #[error("1225 - ERROR_ARANGO_OUT_OF_KEYS")]
    ArangoOutOfKeys,
    /// 1226 - ERROR_ARANGO_DOCUMENT_KEY_MISSING
    ///
    /// Will be raised when a document key is missing.
    #[error("1226 - ERROR_ARANGO_DOCUMENT_KEY_MISSING")]
    ArangoDocumentKeyMissing,
    /// 1227 - ERROR_ARANGO_DOCUMENT_TYPE_INVALID
    ///
    /// Will be raised when there is an attempt to create a document with an invalid type.
    #[error("1227 - ERROR_ARANGO_DOCUMENT_TYPE_INVALID")]
    ArangoDocumentTypeInvalid,
    /// 1228 - ERROR_ARANGO_DATABASE_NOT_FOUND
    ///
    /// Will be raised when a non-existing database is accessed.
    #[error("1228 - ERROR_ARANGO_DATABASE_NOT_FOUND")]
    ArangoDatabaseNotFound,
    /// 1229 - ERROR_ARANGO_DATABASE_NAME_INVALID
    ///
    /// Will be raised when an invalid database name is used.
    #[error("1229 - ERROR_ARANGO_DATABASE_NAME_INVALID")]
    ArangoDatabaseNameInvalid,
    /// 1230 - ERROR_ARANGO_USE_SYSTEM_DATABASE
    ///
    /// Will be raised when an operation is requested in a database other than the system database.
    #[error("1230 - ERROR_ARANGO_USE_SYSTEM_DATABASE")]
    ArangoUseSystemDatabase,
    /// 1232 - ERROR_ARANGO_INVALID_KEY_GENERATOR
    ///
    /// Will be raised when an invalid key generator description is used.
    #[error("1232 - ERROR_ARANGO_INVALID_KEY_GENERATOR")]
    ArangoInvalidKeyGenerator,
    /// 1233 - ERROR_ARANGO_INVALID_EDGE_ATTRIBUTE
    ///
    /// will be raised when the _from or _to values of an edge are undefined or contain an invalid value.
    #[error("1233 - ERROR_ARANGO_INVALID_EDGE_ATTRIBUTE")]
    ArangoInvalidEdgeAttribute,
    /// 1235 - ERROR_ARANGO_INDEX_CREATION_FAILED
    ///
    /// Will be raised when an attempt to create an index has failed.
    #[error("1235 - ERROR_ARANGO_INDEX_CREATION_FAILED")]
    ArangoIndexCreationFailed,
    /// 1236 - ERROR_ARANGO_WRITE_THROTTLE_TIMEOUT
    ///
    /// Will be raised when the server is write-throttled and a write operation has waited too long for the server to process queued operations.
    #[error("1236 - ERROR_ARANGO_WRITE_THROTTLE_TIMEOUT")]
    ArangoWriteThrottleTimeout,
    /// 1237 - ERROR_ARANGO_COLLECTION_TYPE_MISMATCH
    ///
    /// Will be raised when a collection has a different type from what has been expected.
    #[error("1237 - ERROR_ARANGO_COLLECTION_TYPE_MISMATCH")]
    ArangoCollectionTypeMismatch,
    /// 1238 - ERROR_ARANGO_COLLECTION_NOT_LOADED
    ///
    /// Will be raised when a collection is accessed that is not yet loaded.
    #[error("1238 - ERROR_ARANGO_COLLECTION_NOT_LOADED")]
    ArangoCollectionNotLoaded,
    /// 1239 - ERROR_ARANGO_DOCUMENT_REV_BAD
    ///
    /// Will be raised when a document revision is corrupt or is missing where needed.
    #[error("1239 - ERROR_ARANGO_DOCUMENT_REV_BAD")]
    ArangoDocumentRevBad,
    /// 1240 - ERROR_ARANGO_INCOMPLETE_READ
    ///
    /// Will be raised by the storage engine when a read cannot be completed.
    #[error("1240 - ERROR_ARANGO_INCOMPLETE_READ")]
    ArangoIncompleteRead,
    /// 1300 - ERROR_ARANGO_DATAFILE_FULL
    ///
    /// Will be raised when the datafile reaches its limit.
    #[error("1300 - ERROR_ARANGO_DATAFILE_FULL")]
    ArangoDatafileFull,
    /// 1301 - ERROR_ARANGO_EMPTY_DATADIR
    ///
    /// Will be raised when encountering an empty server database directory.
    #[error("1301 - ERROR_ARANGO_EMPTY_DATADIR")]
    ArangoEmptyDatadir,
    /// 1302 - ERROR_ARANGO_TRY_AGAIN
    ///
    /// Will be raised when an operation should be retried.
    #[error("1302 - ERROR_ARANGO_TRY_AGAIN")]
    ArangoTryAgain,
    /// 1303 - ERROR_ARANGO_BUSY
    ///
    /// Will be raised when storage engine is busy.
    #[error("1303 - ERROR_ARANGO_BUSY")]
    ArangoBusy,
    /// 1304 - ERROR_ARANGO_MERGE_IN_PROGRESS
    ///
    /// Will be raised when storage engine has a datafile merge in progress and cannot complete the operation.
    #[error("1304 - ERROR_ARANGO_MERGE_IN_PROGRESS")]
    ArangoMergeInProgress,
    /// 1305 - ERROR_ARANGO_IO_ERROR
    ///
    /// Will be raised when storage engine encounters an I/O error.
    #[error("1305 - ERROR_ARANGO_IO_ERROR")]
    ArangoIoError,
    /// 1400 - ERROR_REPLICATION_NO_RESPONSE
    ///
    /// Will be raised when the replication applier does not receive any or an incomplete response from the master.
    #[error("1400 - ERROR_REPLICATION_NO_RESPONSE")]
    ReplicationNoResponse,
    /// 1401 - ERROR_REPLICATION_INVALID_RESPONSE
    ///
    /// Will be raised when the replication applier receives an invalid response from the master.
    #[error("1401 - ERROR_REPLICATION_INVALID_RESPONSE")]
    ReplicationInvalidResponse,
    /// 1402 - ERROR_REPLICATION_MASTER_ERROR
    ///
    /// Will be raised when the replication applier receives a server error from the master.
    #[error("1402 - ERROR_REPLICATION_MASTER_ERROR")]
    ReplicationMasterError,
    /// 1403 - ERROR_REPLICATION_MASTER_INCOMPATIBLE
    ///
    /// Will be raised when the replication applier connects to a master that has an incompatible version.
    #[error("1403 - ERROR_REPLICATION_MASTER_INCOMPATIBLE")]
    ReplicationMasterIncompatible,
    /// 1404 - ERROR_REPLICATION_MASTER_CHANGE
    ///
    /// Will be raised when the replication applier connects to a different master than before.
    #[error("1404 - ERROR_REPLICATION_MASTER_CHANGE")]
    ReplicationMasterChange,
    /// 1405 - ERROR_REPLICATION_LOOP
    ///
    /// Will be raised when the replication applier is asked to connect to itself for replication.
    #[error("1405 - ERROR_REPLICATION_LOOP")]
    ReplicationLoop,
    /// 1406 - ERROR_REPLICATION_UNEXPECTED_MARKER
    ///
    /// Will be raised when an unexpected marker is found in the replication log stream.
    #[error("1406 - ERROR_REPLICATION_UNEXPECTED_MARKER")]
    ReplicationExpectedMarker,
    /// 1407 - ERROR_REPLICATION_INVALID_APPLIER_STATE
    ///
    /// Will be raised when an invalid replication applier state file is found.
    #[error("1407 - ERROR_REPLICATION_INVALID_APPLIER_STATE")]
    ReplicationInvalidApplierState,
    /// 1408 - ERROR_REPLICATION_UNEXPECTED_TRANSACTION
    ///
    /// Will be raised when an unexpected transaction id is found.
    #[error("1408 - ERROR_REPLICATION_UNEXPECTED_TRANSACTION")]
    ErrorReplicationUnexpectedTransaction,
    /// 1410 - ERROR_REPLICATION_INVALID_APPLIER_CONFIGURATION
    ///
    /// Will be raised when the configuration for the replication applier is invalid.
    #[error("1410 - ERROR_REPLICATION_INVALID_APPLIER_CONFIGURATION")]
    ReplicationInvalidApplierConfiguration,
    /// 1411 - ERROR_REPLICATION_RUNNING
    ///
    /// Will be raised when there is an attempt to perform an operation while the replication applier is running.
    #[error("1411 - ERROR_REPLICATION_RUNNING")]
    ReplicationRunning,
    /// 1412 - ERROR_REPLICATION_APPLIER_STOPPED
    ///
    /// Special error code used to indicate the replication applier was stopped by a user.
    #[error("1412 - ERROR_REPLICATION_APPLIER_STOPPED")]
    ReplicationApplierStopped,
    /// 1413 - ERROR_REPLICATION_NO_START_TICK
    ///
    /// Will be raised when the replication applier is started without a known start tick value.
    #[error("1413 - ERROR_REPLICATION_NO_START_TICK")]
    ReplicationNoStartTick,
    /// 1414 - ERROR_REPLICATION_START_TICK_NOT_PRESENT
    ///
    /// Will be raised when the replication applier fetches data using a start tick
    #[error("1414 - ERROR_REPLICATION_START_TICK_NOT_PRESENT")]
    ReplicationStartTickNotPresent,
    /// 1416 - ERROR_REPLICATION_WRONG_CHECKSUM
    ///
    /// Will be raised when a new born follower submits a wrong checksum
    #[error("1416 - ERROR_REPLICATION_WRONG_CHECKSUM")]
    ReplicationWrongCheckSum,
    /// 1417 - ERROR_REPLICATION_SHARD_NONEMPTY
    ///
    /// Will be raised when a shard is not empty and the follower tries a shortcut
    #[error("1417 - ERROR_REPLICATION_SHARD_NONEMPTY")]
    ReplicationShardNonEmpty,
    /// 1447 - ERROR_CLUSTER_FOLLOWER_TRANSACTION_COMMIT_PERFORMED
    ///
    /// Will be raised when a follower transaction has already performed an intermediate commit and must be rolled back.
    #[error("1447 - ERROR_CLUSTER_FOLLOWER_TRANSACTION_COMMIT_PERFORMED")]
    ClusterFollowerTransactionCommitPerformed,
    /// 1448 - ERROR_CLUSTER_CREATE_COLLECTION_PRECONDITION_FAILED
    ///
    /// Will be raised when updating the plan on collection creation failed.
    #[error("1448 - ERROR_CLUSTER_CREATE_COLLECTION_PRECONDITION_FAILED")]
    ClusterCreateCollectionPreconditionFailed,
    /// 1449 - ERROR_CLUSTER_SERVER_UNKNOWN
    ///
    /// Will be raised on some occasions when one server gets a request from another
    #[error("1449 - ERROR_CLUSTER_SERVER_UNKNOWN")]
    ClusterServerUnknown,
    /// 1450 - ERROR_CLUSTER_TOO_MANY_SHARDS
    ///
    /// Will be raised when the number of shards for a collection is higher than allowed.
    #[error("1450 - ERROR_CLUSTER_TOO_MANY_SHARDS")]
    ClusterTooManyShards,
    /// 1453 - ERROR_CLUSTER_COLLECTION_ID_EXISTS
    ///
    /// Will be raised when a Coordinator in a cluster tries to create a collection and the collection ID already exists.
    #[error("1453 - ERROR_CLUSTER_COLLECTION_ID_EXISTS")]
    ClusterCollectionIdExists,
    /// 1454 - ERROR_CLUSTER_COULD_NOT_CREATE_COLLECTION_IN_PLAN
    ///
    /// Will be raised when a Coordinator in a cluster cannot create an entry for a new collection in the Plan hierarchy in the Agency.
    #[error("1454 - ERROR_CLUSTER_COULD_NOT_CREATE_COLLECTION_IN_PLAN")]
    ClusterCouldNotCreateCollectionInPlan,
    /// 1456 - ERROR_CLUSTER_COULD_NOT_CREATE_COLLECTION
    ///
    /// Will be raised when a Coordinator in a cluster notices that some DB-Servers report problems when creating shards for a new collection.
    #[error("1456 - ERROR_CLUSTER_COULD_NOT_CREATE_COLLECTION")]
    ClusterCouldNotCreateCollection,
    /// 1457 - ERROR_CLUSTER_TIMEOUT
    ///
    /// Will be raised when a Coordinator in a cluster runs into a timeout for some cluster wide operation.
    #[error("1457 - ERROR_CLUSTER_TIMEOUT")]
    ClusterTimeout,
    /// 1458 - ERROR_CLUSTER_COULD_NOT_REMOVE_COLLECTION_IN_PLAN
    ///
    /// Will be raised when a Coordinator in a cluster cannot remove an entry for a collection in the Plan hierarchy in the Agency.
    #[error("1458 - ERROR_CLUSTER_COULD_NOT_REMOVE_COLLECTION_IN_PLAN")]
    ClusterCouldNotRemoveCollectionInPlan,
    /// 1459 - ERROR_CLUSTER_COULD_NOT_REMOVE_COLLECTION_IN_CURRENT
    ///
    /// Will be raised when a Coordinator in a cluster cannot remove an entry for a collection in the Current hierarchy in the Agency.
    #[error("1459 - ERROR_CLUSTER_COULD_NOT_REMOVE_COLLECTION_IN_CURRENT")]
    ClusterCouldNotRemoveCollectionInCurrent,
    /// 1460 - ERROR_CLUSTER_COULD_NOT_CREATE_DATABASE_IN_PLAN
    ///
    /// Will be raised when a Coordinator in a cluster cannot create an entry for a new database in the Plan hierarchy in the Agency.
    #[error("1460 - ERROR_CLUSTER_COULD_NOT_CREATE_DATABASE_IN_PLAN")]
    ClusterCouldNotCreateDatabaseInPlan,
    /// 1461 - ERROR_CLUSTER_COULD_NOT_CREATE_DATABASE
    ///
    /// Will be raised when a Coordinator in a cluster notices that some DB-Servers report problems when creating databases for a new cluster wide database.
    #[error("1461 - ERROR_CLUSTER_COULD_NOT_CREATE_DATABASE")]
    ClusterCouldNotCreateDatabase,
    /// 1462 - ERROR_CLUSTER_COULD_NOT_REMOVE_DATABASE_IN_PLAN
    ///
    /// Will be raised when a Coordinator in a cluster cannot remove an entry for a database in the Plan hierarchy in the Agency.
    #[error("1462 - ERROR_CLUSTER_COULD_NOT_REMOVE_DATABASE_IN_PLAN")]
    ClusterCouldNotRemoveDatabaseInPlan,
    /// 1463 - ERROR_CLUSTER_COULD_NOT_REMOVE_DATABASE_IN_CURRENT
    ///
    /// Will be raised when a Coordinator in a cluster cannot remove an entry for a database in the Current hierarchy in the Agency.
    #[error("1463 - ERROR_CLUSTER_COULD_NOT_REMOVE_DATABASE_IN_CURRENT")]
    ClusterCouldNotRemoveDatabaseInCurrent,
    /// 1464 - ERROR_CLUSTER_SHARD_GONE
    ///
    /// Will be raised when a Coordinator in a cluster cannot determine the shard that is responsible for a given document.
    #[error("1464 - ERROR_CLUSTER_SHARD_GONE")]
    ClusterShardGone,
    /// 1465 - ERROR_CLUSTER_CONNECTION_LOST
    ///
    /// Will be raised when a Coordinator in a cluster loses an HTTP connection to a DB-Server in the cluster whilst transferring data.
    #[error("1465 - ERROR_CLUSTER_CONNECTION_LOST")]
    ClusterConnectionLost,
    /// 1466 - ERROR_CLUSTER_MUST_NOT_SPECIFY_KEY
    ///
    /// Will be raised when a Coordinator in a cluster finds that the _key attribute was specified in a sharded collection the uses not only _key as sharding attribute.
    #[error("1466 - ERROR_CLUSTER_MUST_NOT_SPECIFY_KEY")]
    ClusterMustNotSpecifyKey,
    /// 1467 - ERROR_CLUSTER_GOT_CONTRADICTING_ANSWERS
    ///
    /// Will be raised if a Coordinator in a cluster gets conflicting results from different shards
    #[error("1467 - ERROR_CLUSTER_GOT_CONTRADICTING_ANSWERS")]
    ClusterGotContradictingAnswers,
    /// 1468 - ERROR_CLUSTER_NOT_ALL_SHARDING_ATTRIBUTES_GIVEN
    ///
    /// Will be raised if a Coordinator tries to find out which shard is responsible for a partial document
    #[error("1468 - ERROR_CLUSTER_NOT_ALL_SHARDING_ATTRIBUTES_GIVEN")]
    ClusterNotAllShardingAttributesGiven,
    /// 1469 - ERROR_CLUSTER_MUST_NOT_CHANGE_SHARDING_ATTRIBUTES
    ///
    /// Will be raised if there is an attempt to update the value of a shard attribute.
    #[error("1469 - ERROR_CLUSTER_MUST_NOT_CHANGE_SHARDING_ATTRIBUTES")]
    ClusterMustNotChangeShardingAttributes,
    /// 1470 - ERROR_CLUSTER_UNSUPPORTED
    ///
    /// Will be raised when there is an attempt to carry out an operation that is not supported in the context of a sharded collection.
    #[error("1470 - ERROR_CLUSTER_UNSUPPORTED")]
    ClusterUnsupported,
    /// 1471 - ERROR_CLUSTER_ONLY_ON_COORDINATOR
    ///
    /// Will be raised if there is an attempt to run a Coordinator-only operation on a different type of node.
    #[error("1471 - ERROR_CLUSTER_ONLY_ON_COORDINATOR")]
    ClusterOnlyOnCoordinator,
    /// 1472 - ERROR_CLUSTER_READING_PLAN_AGENCY
    ///
    /// Will be raised if a Coordinator or DB-Server cannot read the Plan in the Agency.
    #[error("1472 - ERROR_CLUSTER_READING_PLAN_AGENCY")]
    ClusterReadingPlanAgency,
    /// 1473 - ERROR_CLUSTER_COULD_NOT_TRUNCATE_COLLECTION
    ///
    /// Will be raised if a Coordinator cannot truncate all shards of a cluster collection.
    #[error("1473 - ERROR_CLUSTER_COULD_NOT_TRUNCATE_COLLECTION")]
    ClusterCouldNotTruncateCollection,
    /// 1474 - ERROR_CLUSTER_AQL_COMMUNICATION
    ///
    /// Will be raised if the internal communication of the cluster for AQL produces an error.
    #[error("1474 - ERROR_CLUSTER_AQL_COMMUNICATION")]
    ClusterAqlCommunication,
    /// 1477 - ERROR_CLUSTER_ONLY_ON_DBSERVER
    ///
    /// Will be raised if there is an attempt to run a DB-Server-only operation on a different type of node.
    #[error("1477 - ERROR_CLUSTER_ONLY_ON_DBSERVER")]
    ClusterOnlyOnDbserver,
    /// 1478 - ERROR_CLUSTER_BACKEND_UNAVAILABLE
    ///
    /// Will be raised if a required DB-Server can’t be reached.
    #[error("1478 - ERROR_CLUSTER_BACKEND_UNAVAILABLE")]
    ClusterBackendUnavailable,
    /// 1481 - ERROR_CLUSTER_AQL_COLLECTION_OUT_OF_SYNC
    ///
    /// Will be raised if a collection needed during query execution is out of sync. This currently can only happen when using SatelliteCollections
    #[error("1481 - ERROR_CLUSTER_AQL_COLLECTION_OUT_OF_SYNC")]
    ClusterAqlCollectionOutOfSync,
    /// 1482 - ERROR_CLUSTER_COULD_NOT_CREATE_INDEX_IN_PLAN
    ///
    /// Will be raised when a Coordinator in a cluster cannot create an entry for a new index in the Plan hierarchy in the Agency.
    #[error("1482 - ERROR_CLUSTER_COULD_NOT_CREATE_INDEX_IN_PLAN")]
    ClusterCouldNotCreateIndexInPlan,
    /// 1483 - ERROR_CLUSTER_COULD_NOT_DROP_INDEX_IN_PLAN
    ///
    /// Will be raised when a Coordinator in a cluster cannot remove an index from the Plan hierarchy in the Agency.
    #[error("1483 - ERROR_CLUSTER_COULD_NOT_DROP_INDEX_IN_PLAN")]
    ClusterCouldNotDropIndexInPlan,
    /// 1484 - ERROR_CLUSTER_CHAIN_OF_DISTRIBUTESHARDSLIKE
    ///
    /// Will be raised if one tries to create a collection with a distributeShardsLike attribute which points to another collection that also has one.
    #[error("1484 - ERROR_CLUSTER_CHAIN_OF_DISTRIBUTESHARDSLIKE")]
    ClusterChainOfDistributeshardslike,
    /// 1485 - ERROR_CLUSTER_MUST_NOT_DROP_COLL_OTHER_DISTRIBUTESHARDSLIKE
    ///
    /// Will be raised if one tries to drop a collection to which another collection points with its distributeShardsLike attribute.
    #[error("1485 - ERROR_CLUSTER_MUST_NOT_DROP_COLL_OTHER_DISTRIBUTESHARDSLIKE")]
    ClusterMustNotDropCollOtherDistributeshardslike,
    /// 1486 - ERROR_CLUSTER_UNKNOWN_DISTRIBUTESHARDSLIKE
    ///
    /// Will be raised if one tries to create a collection which points to an unknown collection in its distributeShardsLike attribute.
    #[error("1486 - ERROR_CLUSTER_UNKNOWN_DISTRIBUTESHARDSLIKE")]
    ClusterUnknownDistributeshardslike,
    /// 1487 - ERROR_CLUSTER_INSUFFICIENT_DBSERVERS
    ///
    /// Will be raised if one tries to create a collection with a replicationFactor greater than the available number of DB-Servers.
    #[error("1487 - ERROR_CLUSTER_INSUFFICIENT_DBSERVERS")]
    ClusterInsufficientDbservers,
    /// 1488 - ERROR_CLUSTER_COULD_NOT_DROP_FOLLOWER
    ///
    /// Will be raised if a follower that ought to be dropped could not be dropped in the Agency (under Current).
    #[error("1488 - ERROR_CLUSTER_COULD_NOT_DROP_FOLLOWER")]
    ClusterCouldNotDropFollower,
    /// 1489 - ERROR_CLUSTER_SHARD_LEADER_REFUSES_REPLICATION
    ///
    /// Will be raised if a replication operation is refused by a shard leader.
    #[error("1489 - ERROR_CLUSTER_SHARD_LEADER_REFUSES_REPLICATION")]
    ClusterShardLeaderRefusesReplication,
    /// 1490 - ERROR_CLUSTER_SHARD_FOLLOWER_REFUSES_OPERATION
    ///
    /// Will be raised if a non-replication operation is refused by a shard follower.
    #[error("1490 - ERROR_CLUSTER_SHARD_FOLLOWER_REFUSES_OPERATION")]
    ClusterShardFollowerRefusesOperation,
    /// 1491 - ERROR_CLUSTER_SHARD_LEADER_RESIGNED
    ///
    /// because it has resigned in the meantime
    #[error("1491 - ERROR_CLUSTER_SHARD_LEADER_RESIGNED")]
    ClusterShardLeaderResigned,
    /// 1492 - ERROR_CLUSTER_AGENCY_COMMUNICATION_FAILED
    ///
    /// Will be raised if after various retries an Agency operation could not be performed successfully.
    #[error("1492 - ERROR_CLUSTER_AGENCY_COMMUNICATION_FAILED")]
    ClusterAgencyCommunicationFailed,
    /// 1495 - ERROR_CLUSTER_LEADERSHIP_CHALLENGE_ONGOING
    ///
    /// Will be raised when servers are currently competing for leadership
    #[error("1495 - ERROR_CLUSTER_LEADERSHIP_CHALLENGE_ONGOING")]
    ClusterLeadershipChallengeOngoing,
    /// 1496 - ERROR_CLUSTER_NOT_LEADER
    ///
    /// Will be raised when an operation is sent to a non-leading server.
    #[error("1496 - ERROR_CLUSTER_NOT_LEADER")]
    ClusterNotLeader,
    /// 1497 - ERROR_CLUSTER_COULD_NOT_CREATE_VIEW_IN_PLAN
    ///
    /// Will be raised when a Coordinator in a cluster cannot create an entry for a new View in the Plan hierarchy in the Agency.
    #[error("1497 - ERROR_CLUSTER_COULD_NOT_CREATE_VIEW_IN_PLAN")]
    ClusterCouldNotCreateViewInPlan,
    /// 1498 - ERROR_CLUSTER_VIEW_ID_EXISTS
    ///
    /// Will be raised when a Coordinator in a cluster tries to create a View and the View ID already exists.
    #[error("1498 - ERROR_CLUSTER_VIEW_ID_EXISTS")]
    ClusterViewIdExists,
    /// 1499 - ERROR_CLUSTER_COULD_NOT_DROP_COLLECTION
    ///
    /// Will be raised when a Coordinator in a cluster cannot drop a collection entry in the Plan hierarchy in the Agency.
    #[error("1499 - ERROR_CLUSTER_COULD_NOT_DROP_COLLECTION")]
    ClusterCouldNotDropCollection,
    /// 1500 - ERROR_QUERY_KILLED
    ///
    /// Will be raised when a running query is killed by an explicit admin command.
    #[error("1500 - ERROR_QUERY_KILLED")]
    QueryKilled,
    /// 1501 - ERROR_QUERY_PARSE
    ///
    /// Will be raised when query is parsed and is found to be syntactically invalid.
    #[error("1501 - ERROR_QUERY_PARSE")]
    QueryParse,
    /// 1502 - ERROR_QUERY_EMPTY
    ///
    /// Will be raised when an empty query is specified.
    #[error("1502 - ERROR_QUERY_EMPTY")]
    QueryEmpty,
    /// 1503 - ERROR_QUERY_SCRIPT
    ///
    /// Will be raised when a runtime error is caused by the query.
    #[error("1503 - ERROR_QUERY_SCRIPT")]
    QueryScript,
    /// 1504 - ERROR_QUERY_NUMBER_OUT_OF_RANGE
    ///
    /// Will be raised when a number is outside the expected range.
    #[error("1504 - ERROR_QUERY_NUMBER_OUT_OF_RANGE")]
    QueryNumberOutOfRange,
    /// 1505 - ERROR_QUERY_INVALID_GEO_VALUE
    ///
    /// Will be raised when a geo index coordinate is invalid or out of range.
    #[error("1505 - ERROR_QUERY_INVALID_GEO_VALUE")]
    QueryInvalidGeoValue,
    /// 1510 - ERROR_QUERY_VARIABLE_NAME_INVALID
    ///
    /// Will be raised when an invalid variable name is used.
    #[error("1510 - ERROR_QUERY_VARIABLE_NAME_INVALID")]
    QueryVariableNameInvalid,
    /// 1511 - ERROR_QUERY_VARIABLE_REDECLARED
    ///
    /// Will be raised when a variable gets re-assigned in a query.
    #[error("1511 - ERROR_QUERY_VARIABLE_REDECLARED")]
    QueryVariableRedeclared,
    /// 1512 - ERROR_QUERY_VARIABLE_NAME_UNKNOWN
    ///
    /// Will be raised when an unknown variable is used or the variable is undefined the context it is used.
    #[error("1512 - ERROR_QUERY_VARIABLE_NAME_UNKNOWN")]
    QueryVariableNameUnknown,
    /// 1521 - ERROR_QUERY_COLLECTION_LOCK_FAILED
    ///
    /// Will be raised when a read lock on the collection cannot be acquired.
    #[error("1521 - ERROR_QUERY_COLLECTION_LOCK_FAILED")]
    QueryCollectionLockFailed,
    /// 1522 - ERROR_QUERY_TOO_MANY_COLLECTIONS
    ///
    /// Will be raised when the number of collections or shards in a query is beyond the allowed value.
    #[error("1522 - ERROR_QUERY_TOO_MANY_COLLECTIONS")]
    QueryTooManyCollections,
    /// 1530 - ERROR_QUERY_DOCUMENT_ATTRIBUTE_REDECLARED
    ///
    /// Will be raised when a document attribute is re-assigned.
    #[error("1530 - ERROR_QUERY_DOCUMENT_ATTRIBUTE_REDECLARED")]
    QueryDocumentAttributeRedeclared,
    /// 1540 - ERROR_QUERY_FUNCTION_NAME_UNKNOWN
    ///
    /// Will be raised when an undefined function is called.
    #[error("1540 - ERROR_QUERY_FUNCTION_NAME_UNKNOWN")]
    QueryFunctionNameUnknown,
    /// 1541 - ERROR_QUERY_FUNCTION_ARGUMENT_NUMBER_MISMATCH
    ///
    /// expected number of arguments: minimum: %d
    #[error("1541 - ERROR_QUERY_FUNCTION_ARGUMENT_NUMBER_MISMATCH")]
    QueryFunctionArgumentNumberMismatch,
    /// 1542 - ERROR_QUERY_FUNCTION_ARGUMENT_TYPE_MISMATCH
    ///
    /// Will be raised when the type of an argument used in a function call does not match the expected argument type.
    #[error("1542 - ERROR_QUERY_FUNCTION_ARGUMENT_TYPE_MISMATCH")]
    QueryFunctionArgumentTypeMismatch,
    /// 1543 - ERROR_QUERY_INVALID_REGEX
    ///
    /// Will be raised when an invalid regex argument value is used in a call to a function that expects a regex.
    #[error("1543 - ERROR_QUERY_INVALID_REGEX")]
    QueryInvalidRegex,
    /// 1550 - ERROR_QUERY_BIND_PARAMETERS_INVALID
    ///
    /// Will be raised when the structure of bind parameters passed has an unexpected format.
    #[error("1550 - ERROR_QUERY_BIND_PARAMETERS_INVALID")]
    QueryBindParametersInvalid,
    /// 1551 - ERROR_QUERY_BIND_PARAMETER_MISSING
    ///
    /// Will be raised when a bind parameter was declared in the query but the query is being executed with no value for that parameter.
    #[error("1551 - ERROR_QUERY_BIND_PARAMETER_MISSING")]
    QueryBindParameterMissing,
    /// 1552 - ERROR_QUERY_BIND_PARAMETER_UNDECLARED
    ///
    /// Will be raised when a value gets specified for an undeclared bind parameter.
    #[error("1552 - ERROR_QUERY_BIND_PARAMETER_UNDECLARED")]
    QueryBindParameterUndeclared,
    /// 1553 - ERROR_QUERY_BIND_PARAMETER_TYPE
    ///
    /// Will be raised when a bind parameter has an invalid value or type.
    #[error("1553 - ERROR_QUERY_BIND_PARAMETER_TYPE")]
    QueryBindParameterType,
    /// 1560 - ERROR_QUERY_INVALID_LOGICAL_VALUE
    ///
    /// Will be raised when a non-boolean value is used in a logical operation.
    #[error("1560 - ERROR_QUERY_INVALID_LOGICAL_VALUE")]
    QueryInvalidLogicalValue,
    /// 1561 - ERROR_QUERY_INVALID_ARITHMETIC_VALUE
    ///
    /// Will be raised when a non-numeric value is used in an arithmetic operation.
    #[error("1561 - ERROR_QUERY_INVALID_ARITHMETIC_VALUE")]
    QueryInvalidArithmeticValue,
    /// 1562 - ERROR_QUERY_DIVISION_BY_ZERO
    ///
    /// Will be raised when there is an attempt to divide by zero.
    #[error("1562 - ERROR_QUERY_DIVISION_BY_ZERO")]
    QueryDivisionByZero,
    /// 1563 - ERROR_QUERY_ARRAY_EXPECTED
    ///
    /// Will be raised when a non-array operand is used for an operation that expects an array argument operand.
    #[error("1563 - ERROR_QUERY_ARRAY_EXPECTED")]
    QueryArrayExpected,
    /// 1569 - ERROR_QUERY_FAIL_CALLED
    ///
    /// Will be raised when the function FAIL() is called from inside a query.
    #[error("1569 - ERROR_QUERY_FAIL_CALLED")]
    QueryFailCalled,
    /// 1570 - ERROR_QUERY_GEO_INDEX_MISSING
    ///
    /// Will be raised when a geo restriction was specified but no suitable geo index is found to resolve it.
    #[error("1570 - ERROR_QUERY_GEO_INDEX_MISSING")]
    QueryGeoIndexMissing,
    /// 1571 - ERROR_QUERY_FULLTEXT_INDEX_MISSING
    ///
    /// Will be raised when a fulltext query is performed on a collection without a suitable fulltext index.
    #[error("1571 - ERROR_QUERY_FULLTEXT_INDEX_MISSING")]
    QueryFulltextIndexMissing,
    /// 1572 - ERROR_QUERY_INVALID_DATE_VALUE
    ///
    /// Will be raised when a value cannot be converted to a date.
    #[error("1572 - ERROR_QUERY_INVALID_DATE_VALUE")]
    QueryInvalidDateValue,
    /// 1573 - ERROR_QUERY_MULTI_MODIFY
    ///
    /// Will be raised when an AQL query contains more than one data-modifying operation.
    #[error("1573 - ERROR_QUERY_MULTI_MODIFY")]
    QueryMultiModify,
    /// 1574 - ERROR_QUERY_INVALID_AGGREGATE_EXPRESSION
    ///
    /// Will be raised when an AQL query contains an invalid aggregate expression.
    #[error("1574 - ERROR_QUERY_INVALID_AGGREGATE_EXPRESSION")]
    QueryInvalidAggregateExpression,
    /// 1575 - ERROR_QUERY_COMPILE_TIME_OPTIONS
    ///
    /// Will be raised when an AQL data-modification query contains options that cannot be figured out at query compile time.
    #[error("1575 - ERROR_QUERY_COMPILE_TIME_OPTIONS")]
    QueryCompileTimeOptions,
    /// 1576 - ERROR_QUERY_EXCEPTION_OPTIONS
    ///
    /// Will be raised when an AQL data-modification query contains an invalid options specification.
    #[error("1576 - ERROR_QUERY_EXCEPTION_OPTIONS")]
    QueryExceptionOptions,
    /// 1577 - ERROR_QUERY_FORCED_INDEX_HINT_UNUSABLE
    ///
    /// Will be raised when forceIndexHint is specified
    #[error("1577 - ERROR_QUERY_FORCED_INDEX_HINT_UNUSABLE")]
    QueryForcedIndexHintUnusable,
    /// 1578 - ERROR_QUERY_DISALLOWED_DYNAMIC_CALL
    ///
    /// Will be raised when a dynamic function call is made to a function that cannot be called dynamically.
    #[error("1578 - ERROR_QUERY_DISALLOWED_DYNAMIC_CALL")]
    QueryDisallowedDynamicCall,
    /// 1579 - ERROR_QUERY_ACCESS_AFTER_MODIFICATION
    ///
    /// Will be raised when collection data are accessed after a data-modification operation.
    #[error("1579 - ERROR_QUERY_ACCESS_AFTER_MODIFICATION")]
    QueryAccessAfterModification,
    /// 1580 - ERROR_QUERY_FUNCTION_INVALID_NAME
    ///
    /// Will be raised when a user function with an invalid name is registered.
    #[error("1580 - ERROR_QUERY_FUNCTION_INVALID_NAME")]
    QueryFunctionInvalidName,
    /// 1581 - ERROR_QUERY_FUNCTION_INVALID_CODE
    ///
    /// Will be raised when a user function is registered with invalid code.
    #[error("1581 - ERROR_QUERY_FUNCTION_INVALID_CODE")]
    QueryFunctionInvalidCode,
    /// 1582 - ERROR_QUERY_FUNCTION_NOT_FOUND
    ///
    /// Will be raised when a user function is accessed but not found.
    #[error("1582 - ERROR_QUERY_FUNCTION_NOT_FOUND")]
    QueryFunctionNotFound,
    /// 1583 - ERROR_QUERY_FUNCTION_RUNTIME_ERROR
    ///
    /// Will be raised when a user function throws a runtime exception.
    #[error("1583 - ERROR_QUERY_FUNCTION_RUNTIME_ERROR")]
    QueryFunctionRuntimeError,
    /// 1590 - ERROR_QUERY_BAD_JSON_PLAN
    ///
    /// Will be raised when an HTTP API for a query got an invalid JSON object.
    #[error("1590 - ERROR_QUERY_BAD_JSON_PLAN")]
    QueryBadJsonPlan,
    /// 1591 - ERROR_QUERY_NOT_FOUND
    ///
    /// Will be raised when an Id of a query is not found by the HTTP API.
    #[error("1591 - ERROR_QUERY_NOT_FOUND")]
    QueryNotFound,
    /// 1593 - ERROR_QUERY_USER_ASSERT
    ///
    /// Will be raised if and user provided expression fails to evaluate to true
    #[error("1593 - ERROR_QUERY_USER_ASSERT")]
    QueryUserAssert,
    /// 1594 - ERROR_QUERY_USER_WARN
    ///
    /// Will be raised if and user provided expression fails to evaluate to true
    #[error("1594 - ERROR_QUERY_USER_WARN")]
    QueryUserWarn,
    /// 1600 - ERROR_CURSOR_NOT_FOUND
    ///
    /// Will be raised when a cursor is requested via its id but a cursor with that id cannot be found.
    #[error("1600 - ERROR_CURSOR_NOT_FOUND")]
    CursorNotFound,
    /// 1601 - ERROR_CURSOR_BUSY
    ///
    /// Will be raised when a cursor is requested via its id but a concurrent request is still using the cursor.
    #[error("1601 - ERROR_CURSOR_BUSY")]
    CursorBusy,
    /// 1620 - ERROR_VALIDATION_FAILED
    ///
    /// Will be raised when a document does not pass schema validation.
    #[error("1620 - ERROR_VALIDATION_FAILED")]
    ValidationFailed,
    /// 1621 - ERROR_VALIDATION_BAD_PARAMETER
    ///
    /// Will be raised when the schema description is invalid.
    #[error("1621 - ERROR_VALIDATION_BAD_PARAMETER")]
    ValidationBadParameter,
    /// 1650 - ERROR_TRANSACTION_INTERNAL
    ///
    /// Will be raised when a wrong usage of transactions is detected. this is an internal error and indicates a bug in `ArangoDB`.
    #[error("1650 - ERROR_TRANSACTION_INTERNAL")]
    TransactionInternalError,
    /// 1651 - ERROR_TRANSACTION_NESTED
    ///
    /// Will be raised when transactions are nested.
    #[error("1651 - ERROR_TRANSACTION_NESTED")]
    TransactionNestedError,
    /// 1652 - ERROR_TRANSACTION_UNREGISTERED_COLLECTION
    ///
    /// Will be raised when a collection is used in the middle of a transaction but was not registered at transaction start.
    #[error("1652 - ERROR_TRANSACTION_UNREGISTERED_COLLECTION")]
    TransactionUnregisteredCollectionError,
    /// 1653 - ERROR_TRANSACTION_DISALLOWED_OPERATION
    ///
    /// Will be raised when a disallowed operation is carried out in a transaction.
    #[error("1653 - ERROR_TRANSACTION_DISALLOWED_OPERATION")]
    TransactionDisallowedOperation,
    /// 1654 - ERROR_TRANSACTION_ABORTED
    ///
    /// Will be raised when a transaction was aborted.
    #[error("1654 - ERROR_TRANSACTION_ABORTED")]
    TransactionAborted,
    /// 1655 - ERROR_TRANSACTION_NOT_FOUND
    ///
    /// Will be raised when a transaction was not found.
    #[error("1655 - ERROR_TRANSACTION_NOT_FOUND")]
    TransactionNotFound,
    /// Unknown error, happens when the error code is not handled
    #[error("Unknown Arango error: `{0}`")]
    UnknownError(u16),
}

impl ArangoError {
    #[must_use]
    #[inline]
    pub(crate) const fn from_error_num(num: u16) -> Self {
        match num {
            1000 => Self::ArangoIllegalState,
            1002 => Self::ArangoDatafileSealed,
            1004 => Self::ArangoReadOnly,
            1005 => Self::ArangoDuplicateIdentifier,
            1006 => Self::ArangoDatafileUnreadable,
            1007 => Self::ArangoDatafileEmpty,
            1008 => Self::ArangoRecovery,
            1009 => Self::ArangoDatafileStatisticsNotFound,
            1100 => Self::ArangoCorruptedDatafile,
            1101 => Self::ArangoIllegalParameterFile,
            1102 => Self::ArangoCorruptedCollection,
            1103 => Self::ArangoMmapFailed,
            1104 => Self::ArangoFileSystemFull,
            1105 => Self::ArangoNoJournal,
            1106 => Self::ArangoDatafileAlreadyExists,
            1107 => Self::ArangoDatadirLocked,
            1108 => Self::ArangoCollectionDirectoryAlreadyExists,
            1109 => Self::ArangoMSyncFailed,
            1110 => Self::ArangoDatadirUnlockable,
            1111 => Self::ArangoSyncTimeout,
            1200 => Self::ArangoConflict,
            1201 => Self::ArangoDatadirInvalid,
            1202 => Self::ArangoDocumentNotFound,
            1203 => Self::ArangoDataSourceNotFound,
            1204 => Self::ArangoCollectionParameterMissing,
            1205 => Self::ArangoDocumentHandleBad,
            1206 => Self::ArangoMaximalSizeTooSmall,
            1207 => Self::ArangoDuplicateName,
            1208 => Self::ArangoIllegalName,
            1209 => Self::ArangoNoIndex,
            1210 => Self::ArangoUniqueConstraintViolated,
            1212 => Self::ArangoIndexNotFound,
            1213 => Self::ArangoCrossCollectionRequest,
            1214 => Self::ArangoIndexHandleBad,
            1216 => Self::ArangoDocumentTooLarge,
            1217 => Self::ArangoCollectionNotUnloaded,
            1218 => Self::ArangoCollectionTypeInvalid,
            1220 => Self::ArangoAttributeParserFailed,
            1221 => Self::ArangoDocumentKeyBad,
            1222 => Self::ArangoDocumentKeyUnexpected,
            1224 => Self::ArangoDatadirNotWritable,
            1225 => Self::ArangoOutOfKeys,
            1226 => Self::ArangoDocumentKeyMissing,
            1227 => Self::ArangoDocumentTypeInvalid,
            1228 => Self::ArangoDatabaseNotFound,
            1229 => Self::ArangoDatabaseNameInvalid,
            1230 => Self::ArangoUseSystemDatabase,
            1232 => Self::ArangoInvalidKeyGenerator,
            1233 => Self::ArangoInvalidEdgeAttribute,
            1235 => Self::ArangoIndexCreationFailed,
            1236 => Self::ArangoWriteThrottleTimeout,
            1237 => Self::ArangoCollectionTypeMismatch,
            1238 => Self::ArangoCollectionNotLoaded,
            1239 => Self::ArangoDocumentRevBad,
            1240 => Self::ArangoIncompleteRead,
            1300 => Self::ArangoDatafileFull,
            1301 => Self::ArangoEmptyDatadir,
            1302 => Self::ArangoTryAgain,
            1303 => Self::ArangoBusy,
            1304 => Self::ArangoMergeInProgress,
            1305 => Self::ArangoIoError,
            1400 => Self::ReplicationNoResponse,
            1401 => Self::ReplicationInvalidResponse,
            1402 => Self::ReplicationMasterError,
            1403 => Self::ReplicationMasterIncompatible,
            1404 => Self::ReplicationMasterChange,
            1405 => Self::ReplicationLoop,
            1406 => Self::ReplicationExpectedMarker,
            1407 => Self::ReplicationInvalidApplierState,
            1408 => Self::ErrorReplicationUnexpectedTransaction,
            1410 => Self::ReplicationInvalidApplierConfiguration,
            1411 => Self::ReplicationRunning,
            1412 => Self::ReplicationApplierStopped,
            1413 => Self::ReplicationNoStartTick,
            1414 => Self::ReplicationStartTickNotPresent,
            1416 => Self::ReplicationWrongCheckSum,
            1417 => Self::ReplicationShardNonEmpty,
            1447 => Self::ClusterFollowerTransactionCommitPerformed,
            1448 => Self::ClusterCreateCollectionPreconditionFailed,
            1449 => Self::ClusterServerUnknown,
            1450 => Self::ClusterTooManyShards,
            1453 => Self::ClusterCollectionIdExists,
            1454 => Self::ClusterCouldNotCreateCollectionInPlan,
            1456 => Self::ClusterCouldNotCreateCollection,
            1457 => Self::ClusterTimeout,
            1458 => Self::ClusterCouldNotRemoveCollectionInPlan,
            1459 => Self::ClusterCouldNotRemoveCollectionInCurrent,
            1460 => Self::ClusterCouldNotCreateDatabaseInPlan,
            1461 => Self::ClusterCouldNotCreateDatabase,
            1462 => Self::ClusterCouldNotRemoveDatabaseInPlan,
            1463 => Self::ClusterCouldNotRemoveDatabaseInCurrent,
            1464 => Self::ClusterShardGone,
            1465 => Self::ClusterConnectionLost,
            1466 => Self::ClusterMustNotSpecifyKey,
            1467 => Self::ClusterGotContradictingAnswers,
            1468 => Self::ClusterNotAllShardingAttributesGiven,
            1469 => Self::ClusterMustNotChangeShardingAttributes,
            1470 => Self::ClusterUnsupported,
            1471 => Self::ClusterOnlyOnCoordinator,
            1472 => Self::ClusterReadingPlanAgency,
            1473 => Self::ClusterCouldNotTruncateCollection,
            1474 => Self::ClusterAqlCommunication,
            1477 => Self::ClusterOnlyOnDbserver,
            1478 => Self::ClusterBackendUnavailable,
            1481 => Self::ClusterAqlCollectionOutOfSync,
            1482 => Self::ClusterCouldNotCreateIndexInPlan,
            1483 => Self::ClusterCouldNotDropIndexInPlan,
            1484 => Self::ClusterChainOfDistributeshardslike,
            1485 => Self::ClusterMustNotDropCollOtherDistributeshardslike,
            1486 => Self::ClusterUnknownDistributeshardslike,
            1487 => Self::ClusterInsufficientDbservers,
            1488 => Self::ClusterCouldNotDropFollower,
            1489 => Self::ClusterShardLeaderRefusesReplication,
            1490 => Self::ClusterShardFollowerRefusesOperation,
            1491 => Self::ClusterShardLeaderResigned,
            1492 => Self::ClusterAgencyCommunicationFailed,
            1495 => Self::ClusterLeadershipChallengeOngoing,
            1496 => Self::ClusterNotLeader,
            1497 => Self::ClusterCouldNotCreateViewInPlan,
            1498 => Self::ClusterViewIdExists,
            1499 => Self::ClusterCouldNotDropCollection,
            1500 => Self::QueryKilled,
            1501 => Self::QueryParse,
            1502 => Self::QueryEmpty,
            1503 => Self::QueryScript,
            1504 => Self::QueryNumberOutOfRange,
            1505 => Self::QueryInvalidGeoValue,
            1510 => Self::QueryVariableNameInvalid,
            1511 => Self::QueryVariableRedeclared,
            1512 => Self::QueryVariableNameUnknown,
            1521 => Self::QueryCollectionLockFailed,
            1522 => Self::QueryTooManyCollections,
            1530 => Self::QueryDocumentAttributeRedeclared,
            1540 => Self::QueryFunctionNameUnknown,
            1541 => Self::QueryFunctionArgumentNumberMismatch,
            1542 => Self::QueryFunctionArgumentTypeMismatch,
            1543 => Self::QueryInvalidRegex,
            1550 => Self::QueryBindParametersInvalid,
            1551 => Self::QueryBindParameterMissing,
            1552 => Self::QueryBindParameterUndeclared,
            1553 => Self::QueryBindParameterType,
            1560 => Self::QueryInvalidLogicalValue,
            1561 => Self::QueryInvalidArithmeticValue,
            1562 => Self::QueryDivisionByZero,
            1563 => Self::QueryArrayExpected,
            1569 => Self::QueryFailCalled,
            1570 => Self::QueryGeoIndexMissing,
            1571 => Self::QueryFulltextIndexMissing,
            1572 => Self::QueryInvalidDateValue,
            1573 => Self::QueryMultiModify,
            1574 => Self::QueryInvalidAggregateExpression,
            1575 => Self::QueryCompileTimeOptions,
            1576 => Self::QueryExceptionOptions,
            1577 => Self::QueryForcedIndexHintUnusable,
            1578 => Self::QueryDisallowedDynamicCall,
            1579 => Self::QueryAccessAfterModification,
            1580 => Self::QueryFunctionInvalidName,
            1581 => Self::QueryFunctionInvalidCode,
            1582 => Self::QueryFunctionNotFound,
            1583 => Self::QueryFunctionRuntimeError,
            1590 => Self::QueryBadJsonPlan,
            1591 => Self::QueryNotFound,
            1593 => Self::QueryUserAssert,
            1594 => Self::QueryUserWarn,
            1600 => Self::CursorNotFound,
            1601 => Self::CursorBusy,
            1620 => Self::ValidationFailed,
            1621 => Self::ValidationBadParameter,
            1650 => Self::TransactionInternalError,
            1651 => Self::TransactionNestedError,
            1652 => Self::TransactionUnregisteredCollectionError,
            1653 => Self::TransactionDisallowedOperation,
            1654 => Self::TransactionAborted,
            1655 => Self::TransactionNotFound,
            _ => Self::UnknownError(num),
        }
    }
}
