/// An 'entity' that exists in a dimension that is not confined to the grid. 
/// It is not a voxel, but it does has position and some shared components.
/// I'm not using the name entity to avoid confusion with the "E" in ECS.
pub struct Element<Id, Display>{
    pub comp_id: Id,
    pub comp_render: Display,
    pub opt_comp: Option<OptionalComps>, //honestly we might remove this
}

#[derive(Debug, Default)]
pub struct OptionalComps(); //Optional list maybe (?)