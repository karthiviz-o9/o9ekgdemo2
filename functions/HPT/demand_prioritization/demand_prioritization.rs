use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DemandType {
    Forecasted,
    Actualized,
}

#[derive(Debug, Clone)]
pub struct Demand {
    pub id: String,
    pub demand_type: DemandType,
    pub priority: u8,
}

pub enum PlannerPreference {
    HighPriorityFirst,
    ActualizedFirst,
}

pub fn process_demands(
    demands: Vec<Demand>,
    preference: PlannerPreference,
) -> Vec<Demand> {
    let forecasted_ids: HashSet<String> = demands
        .iter()
        .filter(|d| d.demand_type == DemandType::Forecasted)
        .map(|d| d.id.clone())
        .collect();

    let mut processed: Vec<Demand> = demands
        .into_iter()
        .filter(|d| {
            if d.demand_type == DemandType::Actualized {
                forecasted_ids.contains(&d.id)
            } else {
                true
            }
        })
        .collect();

    match preference {
        PlannerPreference::HighPriorityFirst => {
            processed.sort_by(|a, b| b.priority.cmp(&a.priority));
        }
        PlannerPreference::ActualizedFirst => {
            processed.sort_by(|a, b| {
                let type_order = |t: &DemandType| match t {
                    DemandType::Actualized => 0,
                    DemandType::Forecasted => 1,
                };
                type_order(&a.demand_type).cmp(&type_order(&b.demand_type))
            });
        }
    }

    processed
}