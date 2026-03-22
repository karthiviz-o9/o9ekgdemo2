use std::collections::HashMap;

pub struct Item {
    pub id: String,
}

pub struct Location {
    pub id: String,
}

pub struct Time {
    pub period: usize,
}

pub struct Lanes {
    pub origin: String,
    pub destination: String,
    pub capacity: f64,
}

pub struct TransportationMode {
    pub id: String,
}

pub struct BillsOfMaterials {
    pub product_id: String,
    pub components: HashMap<String, f64>,
}

pub struct Routings {
    pub product_id: String,
    pub resource_id: String,
    pub time_per_unit: f64,
}

pub struct Resource {
    pub id: String,
    pub capacity: f64,
}

pub struct Network {
    pub lanes: Vec<Lanes>,
    pub resources: Vec<Resource>,
}

pub struct Demand {
    pub item_id: String,
    pub location_id: String,
    pub quantity: f64,
    pub time: usize,
}

pub struct Transactions {
    pub item_id: String,
    pub quantity: f64,
}

pub struct SupplyPlan {
    pub distribution_plan: Vec<String>,
    pub procurement_plan: Vec<String>,
    pub production_plan: Vec<String>,
    pub demand_fulfillment_plan: Vec<String>,
}

pub fn generate_supply_plan(
    items: &[Item],
    locations: &[Location],
    times: &[Time],
    lanes: &[Lanes],
    modes: &[TransportationMode],
    boms: &[BillsOfMaterials],
    routings: &[Routings],
    network: &Network,
    demands: &[Demand],
    inventory: &[Transactions],
) -> Result<SupplyPlan, String> {
    let mut resource_usage: HashMap<String, f64> = HashMap::new();
    let mut lane_usage: HashMap<(String, String), f64> = HashMap::new();

    for demand in demands {
        let routing = routings.iter().find(|r| r.product_id == demand.item_id)
            .ok_or("No routing found for product")?;
        
        let resource = network.resources.iter().find(|r| r.id == routing.resource_id)
            .ok_or("Resource not found")?;

        let current_usage = resource_usage.entry(resource.id.clone()).or_insert(0.0);
        if *current_usage + (demand.quantity * routing.time_per_unit) > resource.capacity {
            return Err("Insufficient resource capacity".to_string());
        }
        *current_usage += demand.quantity * routing.time_per_unit;

        if let Some(bom) = boms.iter().find(|b| b.product_id == demand.item_id) {
            for (comp_id, qty) in &bom.components {
                let stock = inventory.iter().filter(|t| t.item_id == *comp_id).map(|t| t.quantity).sum::<f64>();
                if stock < (qty * demand.quantity) {
                    return Err(format!("Insufficient raw material: {}", comp_id));
                }
            }
        }
    }

    Ok(SupplyPlan {
        distribution_plan: vec!["Distribution initialized".to_string()],
        procurement_plan: vec!["Procurement initialized".to_string()],
        production_plan: vec!["Production initialized".to_string()],
        demand_fulfillment_plan: vec!["Fulfillment verified".to_string()],
    })
}