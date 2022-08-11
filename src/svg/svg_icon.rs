const SVG_DOCUMENT: &[u8] = include_bytes!("../../static/svgs/Document.svg");
const SVG_DOCUMENT_SMALL: &[u8] = include_bytes!("../../static/svgs/Document-small.svg");
const SVG_FOLDER: &[u8] = include_bytes!("../../static/svgs/Folder.svg");
const SVG_FOLDER_SMALL: &[u8] = include_bytes!("../../static/svgs/Folder-small.svg");
const SVG_X: &[u8] = include_bytes!("../../static/svgs/X.svg");
const SVG_X_SMALL: &[u8] = include_bytes!("../../static/svgs/X-small.svg");

#[allow(dead_code)]
pub enum SvgIcon {
    Document,
    DocumentSmall,
    Folder,
    FolderSmall,
    X,
    XSmall,
}

impl SvgIcon {
    pub fn to_bytes(&self) -> &[u8] {
        match self {
            SvgIcon::Document => SVG_DOCUMENT,
            SvgIcon::DocumentSmall => SVG_DOCUMENT_SMALL,
            SvgIcon::Folder => SVG_FOLDER,
            SvgIcon::FolderSmall => SVG_FOLDER_SMALL,
            SvgIcon::X => SVG_X,
            SvgIcon::XSmall => SVG_X_SMALL,
        }
    }
}
