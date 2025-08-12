use afire::{HeaderName, Middleware, Request, Response, prelude::MiddleResult};

pub struct Version;

impl Middleware for Version {
    fn post(&self, _req: &Request, res: &mut Response) -> MiddleResult {
        res.headers.add(
            HeaderName::Server,
            concat!("BeamTime leaderboard v", env!("CARGO_PKG_VERSION")),
        );
        MiddleResult::Continue
    }
}
