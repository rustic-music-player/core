use provider::pocketcasts::PocketcastPodcast;

use reqwest::Client;
use reqwest::header;

const LOGIN_URI: &str = "https://play.pocketcasts.com/users/sign_in";
const GET_SUBSCRIPTIONS_URI: &str = "https://play.pocketcasts.com/web/podcasts/all.json";
const GET_TOP_CHARTS_URI: &str = "https://static.pocketcasts.com/discover/json/popular_world.json";
const GET_FEATURED_URI: &str = "https://static.pocketcasts.com/discover/json/featured.json";
const GET_TRENDING_URI: &str = "https://static.pocketcasts.com/discover/json/trending.json";

#[derive(Debug, Deserialize, Clone)]
pub struct PocketcastUser {
    email: String,
    password: String,
    pub session: Option<String>
}

impl PocketcastUser {
    pub fn login(&mut self) {
        let body = [
            ("[user]email", self.email.as_str()),
            ("[user]password", self.password.as_str())
        ];

        let client = Client::new();
        let res = client.post(LOGIN_URI)
            .form(&body)
            .send()
            .unwrap();

        let _cookies = res.headers().get::<header::SetCookie>().unwrap();
    }

    pub fn get_subscriptions(&self) -> Vec<PocketcastPodcast> {
        let client = Client::new();
        let session = self.session.clone().expect("Login first");
        let mut cookies = header::Cookie::new();
        cookies.set("_social_session", session);
        let mut res = client.post(GET_SUBSCRIPTIONS_URI)
            .header(cookies)
            .send()
            .unwrap();

        if !res.status().is_success() {
            return vec![];
        }

        let res: SubscriptionsResponse = res.json().unwrap();

        res.podcasts
    }

    pub fn get_top_charts(&self) -> Vec<PocketcastPodcast> {
        self.get_discover(GET_TOP_CHARTS_URI)
    }

    pub fn get_featured(&self) -> Vec<PocketcastPodcast> {
        self.get_discover(GET_FEATURED_URI)
    }

    pub fn get_trending(&self) -> Vec<PocketcastPodcast> {
        self.get_discover(GET_TRENDING_URI)
    }

    fn get_discover(&self, uri: &'static str) -> Vec<PocketcastPodcast> {
        let client = Client::new();
        let session = self.session.clone().expect("Login first");
        let mut cookies = header::Cookie::new();
        cookies.set("_social_session", session);
        let mut res = client
            .get(uri)
            .header(cookies)
            .send()
            .unwrap();

        if !res.status().is_success() {
            return vec![];
        }

        let res: DiscoverResponse = res.json().unwrap();

        res.result.unwrap().podcasts
    }
}

#[derive(Debug, Deserialize)]
struct SubscriptionsResponse {
    podcasts: Vec<PocketcastPodcast>
}

#[derive(Debug, Deserialize)]
struct DiscoverResponse {
    result: Option<DiscoverResult>,
    status: String
}

#[derive(Debug, Deserialize)]
struct DiscoverResult {
    podcasts: Vec<PocketcastPodcast>
}
