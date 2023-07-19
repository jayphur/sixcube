#[derive(Default, Debug)]
pub struct Voxel<Id, Display>{
    pub comp_id: Id,
    pub comp_render: Display,
    pub opt_comp: Option<OptionalComps>, //honestly we might remove this
}

#[derive(Debug, Default)]
pub struct OptionalComps(); //Optional list maybe (?)