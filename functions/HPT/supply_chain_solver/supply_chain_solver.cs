public record Time(int Period);
public record Network(string NodeId, List<string> Locations);
public record Demand(string ProductId, int Quantity, Time Period);
public record Transaction(string ProductId, int Quantity, Time Period, string Type);

public record SupplyPlan(
    List<DistributionPlan> DistributionPlans,
    List<ProductionPlan> ProductionPlans,
    List<ProcurementPlan> ProcurementPlans,
    List<DemandFulfillmentPlan> FulfillmentPlans
);

public record DistributionPlan(string ProductId, int Quantity, string From, string To, Time Period);
public record ProductionPlan(string ProductId, int Quantity, Time Period);
public record ProcurementPlan(string ProductId, int Quantity, Time Period);
public record DemandFulfillmentPlan(string ProductId, int Quantity, bool IsFulfilled, Time Period);

public class SupplyPlanner
{
    public SupplyPlan GenerateSupplyPlan(Network network, List<Demand> demands, List<Transaction> transactions)
    {
        var distributionPlans = new List<DistributionPlan>();
        var productionPlans = new List<ProductionPlan>();
        var procurementPlans = new List<ProcurementPlan>();
        var fulfillmentPlans = new List<DemandFulfillmentPlan>();

        foreach (var demand in demands)
        {
            int currentInventory = transactions
                .Where(t => t.ProductId == demand.ProductId && t.Period.Period <= demand.Period.Period)
                .Sum(t => t.Quantity);

            if (currentInventory >= demand.Quantity)
            {
                fulfillmentPlans.Add(new DemandFulfillmentPlan(demand.ProductId, demand.Quantity, true, demand.Period));
            }
            else
            {
                int deficit = demand.Quantity - currentInventory;
                
                procurementPlans.Add(new ProcurementPlan(demand.ProductId, deficit, demand.Period));
                productionPlans.Add(new ProductionPlan(demand.ProductId, deficit, demand.Period));
                
                foreach (var location in network.Locations)
                {
                    distributionPlans.Add(new DistributionPlan(demand.ProductId, deficit / network.Locations.Count, "Central", location, demand.Period));
                }

                fulfillmentPlans.Add(new DemandFulfillmentPlan(demand.ProductId, demand.Quantity, true, demand.Period));
            }
        }

        return new SupplyPlan(distributionPlans, productionPlans, procurementPlans, fulfillmentPlans);
    }
}