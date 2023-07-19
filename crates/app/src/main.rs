pub use type_roster::TypeRoster;

mod type_roster{
    pub struct TypeRoster();
    impl sc_core::types::TypeRoster for TypeRoster{
        type DisplaySystem = ();

        type VoxelDisplay = ();

        type ElementDisplay = ();

        type IdSystem = ();

        type VoxelId = ();

        type ElementId = ();

        type VoxelMap = ();

        type ElementMap = ();
    }
};