//! Fluent message IDs for every owner-facing foundation message (R10, PLAN-26).
//! The wording lives only in `crates/meep-web/locales/en/foundation.ftl`, never in code.

macro_rules! messages {
    ($($variant:ident => $id:literal),+ $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum MessageId {
            $($variant),+
        }

        impl MessageId {
            /// Every message, in the order of contracts/owner-messages.md.
            pub const ALL: &'static [MessageId] = &[$(MessageId::$variant),+];

            /// The Fluent message ID used in the catalog.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(MessageId::$variant => $id),+
                }
            }
        }
    };
}

messages! {
    StoreAlreadyOpen => "store-already-open",
    StoreCreatedRecoveryKey => "store-created-recovery-key",
    StoreNeedsRecoveryKey => "store-needs-recovery-key",
    StoreLostBothKeys => "store-lost-both-keys",
    StoreWrongKey => "store-wrong-key",
    StoreDamagedReadonly => "store-damaged-readonly",
    StoreUnreadable => "store-unreadable",
    HistoryDoesNotVerify => "history-does-not-verify",
    BackupNotEncryptedWarning => "backup-not-encrypted-warning",
    BackupMade => "backup-made",
    BackupChangesAfter => "backup-changes-after",
    BackupMakeAnother => "backup-make-another",
    BackupFailed => "backup-failed",
    LastSuccessfulBackup => "last-successful-backup",
    LastSuccessfulBackupNone => "last-successful-backup-none",
    RestoreNeedsReplaceEverything => "restore-needs-replace-everything",
    RestoreRefused => "restore-refused",
    RestoreVersionTooNew => "restore-version-too-new",
    RestoreRescueWarning => "restore-rescue-warning",
    RestoreNewRecoveryKey => "restore-new-recovery-key",
    AddressInvalidSegments => "address-invalid-segments",
    AddressInvalidSegment => "address-invalid-segment",
    SaveFailed => "save-failed",
    StoreReadOnly => "store-read-only",
    RequestRefused => "request-refused",
    RecordChanged => "record-changed",
    RecordNotFound => "record-not-found",
    RecordUnreadable => "record-unreadable",
    RecordDeleted => "record-deleted",
    InternalError => "internal-error",
}

/// A message to show the owner: its ID and the named values the catalog text uses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerMessage {
    pub id: MessageId,
    pub args: Vec<(&'static str, String)>,
}
