use crate::middleware::rbac::CurrentUser;

/// Evolith-owned request data exposed to GraphQL resolvers.
///
/// Keep transport-library types out of resolver contracts. Domain slices may add established
/// application-service handles here as they migrate, but credentials themselves never belong here.
#[derive(Debug, Clone)]
pub struct GraphQLRequestContext {
    pub request_id: String,
    pub principal: Option<CurrentUser>,
}

impl GraphQLRequestContext {
    pub fn is_authenticated(&self) -> bool {
        self.principal.is_some()
    }
}
