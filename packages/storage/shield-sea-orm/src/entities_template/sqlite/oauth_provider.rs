//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "oauth_provider")]
pub struct Model {
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_type = "Binary(BlobSize::Blob(None))"
    )]
    pub id: Vec<u8>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub created_at: Vec<u8>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub updated_at: Vec<u8>,
    pub name: String,
    pub slug: Option<String>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub r#type: Vec<u8>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub visibility: Vec<u8>,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub scopes: Option<String>,
    pub redirect_url: Option<String>,
    pub authorization_url: Option<String>,
    pub authorization_url_params: Option<String>,
    pub token_url: Option<String>,
    pub token_url_params: Option<String>,
    pub introspection_url: Option<String>,
    pub introspection_url_params: Option<String>,
    pub revocation_url: Option<String>,
    pub revocation_url_params: Option<String>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub pkce_code_challenge: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::oauth_provider_connection::Entity")]
    OauthProviderConnection,
}

impl Related<super::oauth_provider_connection::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OauthProviderConnection.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}