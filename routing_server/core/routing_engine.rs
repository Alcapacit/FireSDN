use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Route {
    pub id: String,
    pub latency_ms: u32,
    pub load_percent: u8,
}

pub struct RoutingEngine {
    pub routes: HashMap<String, Route>,
}

impl RoutingEngine {
    pub fn new() -> Self {
        RoutingEngine {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, route: Route) {
        self.routes.insert(route.id.clone(), route);
    }

    pub fn select_best_route(&self) -> Option<&Route> {
        self.routes.values().min_by_key(|r| r.latency_ms)
    }
}