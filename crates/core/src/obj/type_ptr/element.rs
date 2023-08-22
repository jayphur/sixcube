use crate::{obj::element::{ElementType, Element}, r#type::StaticType};


#[derive(Debug)]
pub struct ElementTypePtr(&'static dyn ElementType);
impl ElementType for ElementTypePtr{

}
impl StaticType for ElementTypePtr{
    type Obj = Element;

    fn name(&'static self) -> &crate::Name {
        todo!()
    }

    fn new_obj(&'static self) -> Self::Obj {
        Element{
            my_type: ElementTypePtr(self.0),
            comp_opt: None,
        }
    }
}