#[derive(Clone,Debug)]
pub struct StationCheckItemNew {
    pub checkuuid: Option<String>,
    pub station_uuid: String,
    pub source: String,
    pub codec: String,
    pub bitrate: u32,
    pub hls: bool,
    pub check_ok: bool,
    pub url: String,
    pub timestamp: Option<String>,

    pub metainfo_overrides_database: bool,
    pub public: Option<bool>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub tags: Option<String>,
    pub countrycode: Option<String>,
    pub homepage: Option<String>,
    pub favicon: Option<String>,
    pub loadbalancer: Option<String>,
    pub do_not_index: Option<bool>,
}