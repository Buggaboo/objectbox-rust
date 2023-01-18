#![allow(non_upper_case_globals)]

// grep from the generated c-bindings
pub type OBXFeature = u32;
pub type OBXLogLevel = u32;
pub type OBXPropertyType = u32;
pub type OBXEntityFlags = u32;
pub type OBXPropertyFlags = u32;
pub type OBXDebugFlags = u32;
pub type OBXPutPaddingMode = u32;
pub type OBXPutMode = u32;
pub type OBXOrderFlags = u32;

// pub const OBX_VERSION_MAJOR: u32 = 0;
// pub const OBX_VERSION_MINOR: u32 = 18;
// pub const OBX_VERSION_PATCH: u32 = 0;
// pub const OBX_ID_NEW: i32 = -1;
// pub const OBX_SUCCESS: u32 = 0;
// pub const OBX_NOT_FOUND: u32 = 404;
// pub const OBX_NO_SUCCESS: u32 = 1001;
// pub const OBX_TIMEOUT: u32 = 1002;
// pub const OBX_ERROR_ILLEGAL_STATE: u32 = 10001;
// pub const OBX_ERROR_ILLEGAL_ARGUMENT: u32 = 10002;
// pub const OBX_ERROR_ALLOCATION: u32 = 10003;
// pub const OBX_ERROR_NUMERIC_OVERFLOW: u32 = 10004;
// pub const OBX_ERROR_FEATURE_NOT_AVAILABLE: u32 = 10005;
// pub const OBX_ERROR_SHUTTING_DOWN: u32 = 10006;
// pub const OBX_ERROR_NO_ERROR_INFO: u32 = 10097;
// pub const OBX_ERROR_GENERAL: u32 = 10098;
// pub const OBX_ERROR_UNKNOWN: u32 = 10099;
// pub const OBX_ERROR_DB_FULL: u32 = 10101;
// pub const OBX_ERROR_MAX_READERS_EXCEEDED: u32 = 10102;
// pub const OBX_ERROR_STORE_MUST_SHUTDOWN: u32 = 10103;
// pub const OBX_ERROR_MAX_DATA_SIZE_EXCEEDED: u32 = 10104;
// pub const OBX_ERROR_DB_GENERAL: u32 = 10198;
// pub const OBX_ERROR_STORAGE_GENERAL: u32 = 10199;
// pub const OBX_ERROR_UNIQUE_VIOLATED: u32 = 10201;
// pub const OBX_ERROR_NON_UNIQUE_RESULT: u32 = 10202;
// pub const OBX_ERROR_PROPERTY_TYPE_MISMATCH: u32 = 10203;
// pub const OBX_ERROR_ID_ALREADY_EXISTS: u32 = 10210;
// pub const OBX_ERROR_ID_NOT_FOUND: u32 = 10211;
// pub const OBX_ERROR_TIME_SERIES: u32 = 10212;
// pub const OBX_ERROR_CONSTRAINT_VIOLATED: u32 = 10299;
// pub const OBX_ERROR_STD_ILLEGAL_ARGUMENT: u32 = 10301;
// pub const OBX_ERROR_STD_OUT_OF_RANGE: u32 = 10302;
// pub const OBX_ERROR_STD_LENGTH: u32 = 10303;
// pub const OBX_ERROR_STD_BAD_ALLOC: u32 = 10304;
// pub const OBX_ERROR_STD_RANGE: u32 = 10305;
// pub const OBX_ERROR_STD_OVERFLOW: u32 = 10306;
// pub const OBX_ERROR_STD_OTHER: u32 = 10399;
// pub const OBX_ERROR_SCHEMA: u32 = 10501;
// pub const OBX_ERROR_FILE_CORRUPT: u32 = 10502;
// pub const OBX_ERROR_FILE_PAGES_CORRUPT: u32 = 10503;
// pub const OBX_ERROR_SCHEMA_OBJECT_NOT_FOUND: u32 = 10504;
// pub const OBX_ERROR_TREE_MODEL_INVALID: u32 = 10601;
// pub const OBX_ERROR_TREE_VALUE_TYPE_MISMATCH: u32 = 10602;
// pub const OBX_ERROR_TREE_PATH_NON_UNIQUE: u32 = 10603;
// pub const OBX_ERROR_TREE_PATH_ILLEGAL: u32 = 10604;
// pub const OBX_ERROR_TREE_OTHER: u32 = 10699;
// pub const OBXFeature_ResultArray: OBXFeature = 1;
// pub const OBXFeature_TimeSeries: OBXFeature = 2;
// pub const OBXFeature_Sync: OBXFeature = 3;
// pub const OBXFeature_DebugLog: OBXFeature = 4;
// pub const OBXFeature_Admin: OBXFeature = 5;
// pub const OBXFeature_Tree: OBXFeature = 6;
// pub const OBXFeature_SyncServer: OBXFeature = 7;
// pub const OBXLogLevel_Verbose: OBXLogLevel = 10;
// pub const OBXLogLevel_Debug: OBXLogLevel = 20;
// pub const OBXLogLevel_Info: OBXLogLevel = 30;
// pub const OBXLogLevel_Warn: OBXLogLevel = 40;
// pub const OBXLogLevel_Error: OBXLogLevel = 50;
pub const OBXPropertyType_Bool: OBXPropertyType = 1;
pub const OBXPropertyType_Byte: OBXPropertyType = 2;
pub const OBXPropertyType_Short: OBXPropertyType = 3;
pub const OBXPropertyType_Char: OBXPropertyType = 4;
pub const OBXPropertyType_Int: OBXPropertyType = 5;
pub const OBXPropertyType_Long: OBXPropertyType = 6;
pub const OBXPropertyType_Float: OBXPropertyType = 7;
pub const OBXPropertyType_Double: OBXPropertyType = 8;
pub const OBXPropertyType_String: OBXPropertyType = 9;
pub const OBXPropertyType_Date: OBXPropertyType = 10;
pub const OBXPropertyType_Relation: OBXPropertyType = 11;
pub const OBXPropertyType_DateNano: OBXPropertyType = 12;
pub const OBXPropertyType_Flex: OBXPropertyType = 13;
pub const OBXPropertyType_ByteVector: OBXPropertyType = 23;
pub const OBXPropertyType_StringVector: OBXPropertyType = 30;
pub const OBXEntityFlags_SYNC_ENABLED: OBXEntityFlags = 2;
pub const OBXEntityFlags_SHARED_GLOBAL_IDS: OBXEntityFlags = 4;
pub const OBXPropertyFlags_ID: OBXPropertyFlags = 1;
pub const OBXPropertyFlags_NON_PRIMITIVE_TYPE: OBXPropertyFlags = 2;
pub const OBXPropertyFlags_NOT_NULL: OBXPropertyFlags = 4;
pub const OBXPropertyFlags_INDEXED: OBXPropertyFlags = 8;
pub const OBXPropertyFlags_RESERVED: OBXPropertyFlags = 16;
pub const OBXPropertyFlags_UNIQUE: OBXPropertyFlags = 32;
pub const OBXPropertyFlags_ID_MONOTONIC_SEQUENCE: OBXPropertyFlags = 64;
pub const OBXPropertyFlags_ID_SELF_ASSIGNABLE: OBXPropertyFlags = 128;
pub const OBXPropertyFlags_INDEX_PARTIAL_SKIP_NULL: OBXPropertyFlags = 256;
pub const OBXPropertyFlags_INDEX_PARTIAL_SKIP_ZERO: OBXPropertyFlags = 512;
pub const OBXPropertyFlags_VIRTUAL: OBXPropertyFlags = 1024;
pub const OBXPropertyFlags_INDEX_HASH: OBXPropertyFlags = 2048;
pub const OBXPropertyFlags_INDEX_HASH64: OBXPropertyFlags = 4096;
pub const OBXPropertyFlags_UNSIGNED: OBXPropertyFlags = 8192;
pub const OBXPropertyFlags_ID_COMPANION: OBXPropertyFlags = 16384;
pub const OBXPropertyFlags_UNIQUE_ON_CONFLICT_REPLACE: OBXPropertyFlags = 32768;
pub const OBXPropertyFlags_EXPIRATION_TIME: OBXPropertyFlags = 65536;
// pub const OBXDebugFlags_LOG_TRANSACTIONS_READ: OBXDebugFlags = 1;
// pub const OBXDebugFlags_LOG_TRANSACTIONS_WRITE: OBXDebugFlags = 2;
// pub const OBXDebugFlags_LOG_QUERIES: OBXDebugFlags = 4;
// pub const OBXDebugFlags_LOG_QUERY_PARAMETERS: OBXDebugFlags = 8;
// pub const OBXDebugFlags_LOG_ASYNC_QUEUE: OBXDebugFlags = 16;
// pub const OBXDebugFlags_LOG_CACHE_HITS: OBXDebugFlags = 32;
// pub const OBXDebugFlags_LOG_CACHE_ALL: OBXDebugFlags = 64;
// pub const OBXDebugFlags_LOG_TREE: OBXDebugFlags = 128;
// pub const OBXDebugFlags_LOG_EXCEPTION_STACK_TRACE: OBXDebugFlags = 256;
// pub const OBXDebugFlags_RUN_THREADING_SELF_TEST: OBXDebugFlags = 512;
// pub const OBXPutPaddingMode_PaddingAutomatic: OBXPutPaddingMode = 1;
// pub const OBXPutPaddingMode_PaddingAllowedByBuffer: OBXPutPaddingMode = 2;
// pub const OBXPutPaddingMode_PaddingByCaller: OBXPutPaddingMode = 3;
// pub const OBXPutMode_PUT: OBXPutMode = 1;
// pub const OBXPutMode_INSERT: OBXPutMode = 2;
// pub const OBXPutMode_UPDATE: OBXPutMode = 3;
// pub const OBXOrderFlags_DESCENDING: OBXOrderFlags = 1;
// pub const OBXOrderFlags_CASE_SENSITIVE: OBXOrderFlags = 2;
// pub const OBXOrderFlags_UNSIGNED: OBXOrderFlags = 4;
// pub const OBXOrderFlags_NULLS_LAST: OBXOrderFlags = 8;
// pub const OBXOrderFlags_NULLS_ZERO: OBXOrderFlags = 16;
