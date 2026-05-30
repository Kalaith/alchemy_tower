pub(crate) struct ShopOverlayView {
    pub(crate) station_name: String,
    pub(crate) coin_count: u32,
    pub(crate) buy_tab_active: bool,
    pub(crate) sell_tab_active: bool,
    pub(crate) stock_title: String,
    pub(crate) sort_label: String,
    pub(crate) empty_text: String,
    pub(crate) safe_sell_banner: Option<String>,
    pub(crate) entries: Vec<ShopOverlayEntry>,
}

pub(crate) struct ShopOverlayEntry {
    pub(crate) title: String,
    pub(crate) detail: String,
    pub(crate) meta: String,
    pub(crate) enabled: bool,
    pub(crate) selected: bool,
}
