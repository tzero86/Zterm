use crate::{error::UserFacingError, schema};

#[derive(cynic::QueryVariables, Debug)]
pub struct ListZtermDevImagesVariables {}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "RootQuery", variables = "ListZtermDevImagesVariables")]
pub struct ListZtermDevImages {
    #[cynic(rename = "listZtermDevImages")]
    pub list_zterm_dev_images: ListZtermDevImagesResult,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct ListZtermDevImagesOutput {
    pub images: Vec<ImageTag>,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct ImageTag {
    pub image: String,
    pub repository: String,
    pub tag: String,
}

#[derive(cynic::InlineFragments, Debug)]
pub enum ListZtermDevImagesResult {
    ListZtermDevImagesOutput(ListZtermDevImagesOutput),
    UserFacingError(UserFacingError),
    #[cynic(fallback)]
    Unknown,
}

crate::client::define_operation! {
    ListZtermDevImages(ListZtermDevImagesVariables) -> ListZtermDevImages;
}
