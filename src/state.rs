use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use reqwest::Client;
use sqlx::SqlitePool;
use tokio::sync::Mutex;

pub struct AppState {
    pub(crate) client: Client,
    pub(crate) pool: SqlitePool,
    pub(crate) rng: Mutex<ChaCha20Rng>,
}

impl AppState {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            client: Client::new(),
            pool,
            rng: Mutex::new(ChaCha20Rng::from_entropy()),
        }
    }
}
