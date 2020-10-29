#[derive(Default, Debug)]
pub struct Application {
    pub id: String,
    pub name: String,
    /// 应用图标
    pub icon_small: Option<String>,
    pub icon_medium: Option<String>,
    pub icon_large: Option<String>,
}
