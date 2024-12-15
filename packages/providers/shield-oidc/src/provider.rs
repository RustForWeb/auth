use async_trait::async_trait;
use openidconnect::{
    core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    AuthUrl, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce, Scope, TokenUrl, UserInfoUrl,
};
use shield::{
    Provider, SignInError, SignInRequest, SignOutError, SignOutRequest, StorageError, Subprovider,
};

use crate::{storage::OidcStorage, subprovider::OidcSubprovider};

pub const OIDC_PROVIDER_ID: &str = "oidc";

#[derive(Default)]
pub struct OidcProvider {
    subproviders: Vec<OidcSubprovider>,
    storage: Option<Box<dyn OidcStorage>>,
}

impl OidcProvider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_storage<S: OidcStorage + 'static>(mut self, storage: S) -> Self {
        self.storage = Some(Box::new(storage));
        self
    }

    pub fn with_subproviders<I: IntoIterator<Item = OidcSubprovider>>(
        mut self,
        subproviders: I,
    ) -> Self {
        self.subproviders = subproviders.into_iter().collect();
        self
    }

    async fn oidc_subprovider_by_id(
        &self,
        subprovider_id: &str,
    ) -> Result<Option<OidcSubprovider>, StorageError> {
        if let Some(subprovider) = self
            .subproviders
            .iter()
            .find(|subprovider| subprovider.id == subprovider_id)
        {
            return Ok(Some(subprovider.clone()));
        }

        if let Some(storage) = &self.storage {
            if let Some(subprovider) = storage.oidc_subprovider_by_id(subprovider_id).await? {
                return Ok(Some(subprovider));
            }
        }

        Ok(None)
    }
}

#[async_trait]
impl Provider for OidcProvider {
    fn id(&self) -> String {
        OIDC_PROVIDER_ID.to_owned()
    }

    async fn subproviders(&self) -> Result<Vec<Box<dyn Subprovider>>, StorageError> {
        let subproviders =
            self.subproviders
                .iter()
                .cloned()
                .chain(if let Some(storage) = &self.storage {
                    storage.oidc_subproviders().await?
                } else {
                    vec![]
                });

        Ok(subproviders
            .map(|subprovider| Box::new(subprovider) as Box<dyn Subprovider>)
            .collect())
    }

    async fn subprovider_by_id(
        &self,
        subprovider_id: &str,
    ) -> Result<Option<Box<dyn Subprovider>>, StorageError> {
        self.oidc_subprovider_by_id(subprovider_id)
            .await
            .map(|subprovider| {
                subprovider.map(|subprovider| Box::new(subprovider) as Box<dyn Subprovider>)
            })
    }

    async fn sign_in(&self, request: SignInRequest) -> Result<(), SignInError> {
        let subprovider = match request.subprovider_id {
            Some(subprovider_id) => match self.oidc_subprovider_by_id(&subprovider_id).await? {
                Some(subprovider) => subprovider,
                None => return Err(SignInError::SubproviderNotFound(subprovider_id)),
            },
            None => return Err(SignInError::SubproviderMissing),
        };

        let client = if let Some(discovery_url) = subprovider.discovery_url {
            let provider_metadata = CoreProviderMetadata::discover_async(
                // TODO: Consider stripping `/.well-known/openid-configuration` so `openidconnect` doesn't error.
                IssuerUrl::new(discovery_url).expect("TODO: issuer url error"),
                async_http_client,
            )
            .await
            .expect("TODO: provider metadata error");

            CoreClient::from_provider_metadata(
                provider_metadata,
                ClientId::new(subprovider.client_id),
                subprovider.client_secret.map(ClientSecret::new),
            )
        } else {
            CoreClient::new(
                ClientId::new(subprovider.client_id),
                subprovider.client_secret.map(ClientSecret::new),
                IssuerUrl::new(subprovider.issuer_url.expect("TODO: missing issuer url"))
                    .expect("TODO: issuer url error"),
                subprovider
                    .authorization_url
                    .map(|authorization_url| {
                        AuthUrl::new(authorization_url).expect("TODO: auth url error")
                    })
                    .expect("TODO: missing authorization url"),
                subprovider
                    .token_url
                    .map(|token_url| TokenUrl::new(token_url).expect("TODO: token url error")),
                subprovider.user_info_url.map(|user_info_url| {
                    UserInfoUrl::new(user_info_url).expect("TODO: user info url error")
                }),
                subprovider.json_web_key_set.expect("TODO: missing jwks"),
            )
        };

        // TODO: Common client options.

        let mut authorization_request = client.authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        );

        // TODO: PKCE code challenge.

        if let Some(scopes) = subprovider.scopes {
            authorization_request =
                authorization_request.add_scopes(scopes.into_iter().map(Scope::new));
        }

        let (auth_url, csrf_token, nonce) = authorization_request.url();

        // TODO: Store CSRF and nonce in session.
        // TODO: Redirect.

        todo!("redirect {} {:?} {:?}", auth_url, csrf_token, nonce)
    }

    async fn sign_out(&self, request: SignOutRequest) -> Result<(), SignOutError> {
        let _subprovider = match request.subprovider_id {
            Some(subprovider_id) => match self.subprovider_by_id(&subprovider_id).await? {
                Some(subprovider) => Some(subprovider),
                None => return Err(SignOutError::SubproviderNotFound(subprovider_id)),
            },
            None => return Err(SignOutError::SubproviderMissing),
        };

        todo!("oidc sign out")
    }
}
