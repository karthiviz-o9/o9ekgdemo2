use std::collections::HashMap;

pub struct Item {
    pub id: String,
    pub name: String,
}

pub struct Location {
    pub id: String,
    pub name: String,
}

pub struct Lanes {
    pub origin_id: String,
    pub destination_id: String,
    pub mode: TransportationMode,
    pub lead_time_days: u32,
}

pub enum TransportationMode {
    Road,
    Rail,
    Air,
    Sea,
}

pub struct BillOfMaterials {
    pub parent_item_id: String,
    pub components: Vec<(String, f64)>,
}

pub struct Routing {
    pub item_id: String,
    pub steps: Vec<RoutingStep>,
}

pub struct RoutingStep {
    pub resource_id: String,
    pub duration_hours: f64,
}

pub struct Resource {
    pub id: String,
    pub location_id: String,
    pub capacity: f64,
}

pub struct Network {
    pub items: HashMap<String, Item>,
    pub locations: HashMap<String, Location>,
    pub lanes: Vec<Lanes>,
    pub boms: HashMap<String, BillOfMaterials>,
    pub routings: HashMap<String, Routing>,
    pub resources: HashMap<String, Resource>,
}

impl Network {
    pub fn new(
        items: Vec<Item>,
        locations: Vec<Location>,
        lanes: Vec<Lanes>,
        boms: Vec<BillOfMaterials>,
        routings: Vec<Routing>,
        resources: Vec<Resource>,
    ) -> Self {
        Self {
            items: items.into_iter().map(|i| (i.id.clone(), i)).collect(),
            locations: locations.into_iter().map(|l| (l.id.clone(), l)).collect(),
            lanes,
            boms: boms.into_iter().map(|b| (b.parent_item_id.clone(), b)).collect(),
            routings: routings.into_iter().map(|r| (r.item_id.clone(), r)).collect(),
            resources: resources.into_iter().map(|r| (r.id.clone(), r)).collect(),
        }
    }
}