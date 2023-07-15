use super::{Build, DimType, Override};

#[derive(Debug, Clone, Default)]
pub struct DimTypeInfo {}
impl Build<DimType> for DimTypeInfo {
    fn build(self) -> DimType {
        DimType {}
    }
}
impl Override<DimType> for DimTypeInfo {
    fn edit(&self, t: DimType) -> DimType {
        t
    }
}
