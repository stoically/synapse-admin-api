//! [GET /_synapse/admin/v2/users/:user_id](https://github.com/matrix-org/synapse/blob/master/docs/admin_api/user_admin_api.rst#query-user-account)

pub use crate::users::UserDetails;
use ruma::{api::ruma_api, UserId};

ruma_api! {
    metadata: {
        description: "Get information about a specific user account.",
        method: GET,
        name: "query_user_account_v2",
        unstable_path: "/_synapse/admin/v2/users/:user_id",
        rate_limited: false,
        authentication: AccessToken,
    }

    request: {
        /// user ID
        #[ruma_api(path)]
        pub user_id: &'a UserId,
    }

    response: {
        /// Details about the user.
        #[ruma_api(body)]
        pub details: UserDetails,
    }
}

impl<'a> Request<'a> {
    /// Creates an `Request` with the given user ID.
    pub fn new(user_id: &'a UserId) -> Self {
        Self { user_id }
    }
}

impl Response {
    /// Creates a new `Response` with all parameters defaulted.
    pub fn new(details: UserDetails) -> Self {
        Self { details }
    }
}
