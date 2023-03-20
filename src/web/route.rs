use yew_router::Routable;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    #[not_found]
    NoRoute,

    #[at("/landing")]
    Landing,
    #[at("/question")]
    Question,

    #[at("/spread")]
    Spread,

    #[at("/advanced")]
    Advanced,

    #[at("/share")]
    Share,

    #[at("/preferences")]
    Preferences,

    #[at("/custom/:cards")]
    Custom { cards: String },

    #[at("/cheat/:cards")]
    Cheat { cards: String },
}

#[cfg(test)]
pub mod tests {
    use yew_router::Routable;

    use crate::web::prelude::Route;

    #[test]
    pub fn test_recognize() {
        assert_eq!(Route::recognize("/landing"), Some(Route::Landing));
        assert_eq!(Route::recognize("/share"), Some(Route::Share));
    }
}
