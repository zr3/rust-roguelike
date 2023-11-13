use rltk::rex::XpFile;

rltk::embedded_resource!(TITLE_BACKGROUND, "../resources/title.xp");

pub struct RexAssets {
    pub menu: XpFile,
}

impl RexAssets {
    pub fn new() -> RexAssets {
        rltk::link_resource!(TITLE_BACKGROUND, "../resources/title.xp");

        RexAssets {
            menu: XpFile::from_resource("../resources/title.xp")
                .expect("should be able to load title xp file"),
        }
    }
}
