pub(super) const ARCHIVE_TABS: [&str; 6] = [
    "timeline",
    "experiments",
    "mastery",
    "morphs",
    "disassembly",
    "duplication",
];

#[derive(Clone, Debug, Default)]
pub(super) struct OverlayState {
    pub(super) journal_tab: usize,
    pub(super) shop_buy_tab: bool,
    pub(super) shop_index: usize,
    pub(super) rune_index: usize,
    pub(super) archive_tab: usize,
    pub(super) archive_index: usize,
    pub(super) archive_experiment_filter: ArchiveExperimentFilter,
    pub(super) current: Option<OverlayScreen>,
    pub(super) inventory_sort_mode: InventorySortMode,
}

impl OverlayState {
    pub(super) fn new_gameplay() -> Self {
        Self {
            shop_buy_tab: true,
            inventory_sort_mode: InventorySortMode::Priority,
            archive_experiment_filter: ArchiveExperimentFilter::All,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum OverlayScreen {
    Journal,
    QuestBoard,
    Shop,
    Rune,
    Archive,
    Ending,
    Dialogue(String),
    Alchemy,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) enum InventorySortMode {
    #[default]
    Priority,
    Type,
    Name,
}

impl InventorySortMode {
    pub(super) fn next(self) -> Self {
        match self {
            Self::Priority => Self::Type,
            Self::Type => Self::Name,
            Self::Name => Self::Priority,
        }
    }

    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Priority => "priority",
            Self::Type => "type",
            Self::Name => "name",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) enum ArchiveExperimentFilter {
    #[default]
    All,
    Stable,
    Unstable,
}

impl ArchiveExperimentFilter {
    pub(super) fn next(self) -> Self {
        match self {
            Self::All => Self::Stable,
            Self::Stable => Self::Unstable,
            Self::Unstable => Self::All,
        }
    }

    pub(super) fn label(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Stable => "stable",
            Self::Unstable => "unstable",
        }
    }
}
