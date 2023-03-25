
/// units leave behind phermones on move.
/// Units searches near fields for pheromones,
/// and if they detect some, that did not originate by themself,
/// then they follow this tracks. 
pub struct PheromoneSystem {
    field_index: usize,    
}

impl PheromoneSystemUnitPlaner {

    pub fn get_field(context: &TickaContext) -> &Field2D<f32> {
        // context.
        // context.
    }
    pub fn get_unit_plan(unit: &Unit, context: &TickaContext) -> UnitPlan {

    }
}

impl PheromoneSystem {
    pub fn new() -> Self {
        PheromoneSystem {
            
        }
    }

    pub fn init(&mut self, ticka: &mut Ticka) {
        self.field_index = ticka.register_field_f32();
    }

    pub

}