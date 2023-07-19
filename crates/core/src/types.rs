use crate::{obj::{dim::{VoxMapTrait, ElemMapTrait}, voxel::Voxel, element::Element}, ecs::{Component, System}};

// this trait is in an awkward middle ground of bi-directional dependency

/// Types defined outside of this crate, define once for easy use.
pub trait TypeRoster{
    // display system
    type DisplaySystem: System;
    type VoxelDisplay: Component<Self::DisplaySystem>;
    type ElementDisplay: Component<Self::DisplaySystem>;

    // id system
    type IdSystem: System;
    type VoxelId: Component<Self::IdSystem>;
    type ElementId: Component<Self::IdSystem>;

    // map
    type VoxelMap: VoxMapTrait<Voxel<Self::VoxelId, Self::VoxelDisplay>>;
    type ElementMap: ElemMapTrait<Element<Self::VoxelId, Self::VoxelDisplay>>;
}