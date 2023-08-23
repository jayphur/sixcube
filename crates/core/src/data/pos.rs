pub struct GlobalPos(i16, i16, i16);
impl GlobalPos {
    pub fn relative(&self) -> RelativePos {
        RelativePos(self.0, self.1, self.2)
    }
    pub fn relative_to(&self, interval: i16) -> RelativePos {
        RelativePos(self.0 % interval, self.1 % interval, self.2 % interval)
    }
    pub fn tuple(&self) -> (i16, i16, i16) {
        (self.0, self.1, self.2)
    }
}
pub struct RelativePos(i16, i16, i16);

pub struct GlobalPosF(f32, f32, f32);
impl GlobalPosF {
    pub fn relative(&self) -> RelativePosF {
        RelativePosF(self.0, self.1, self.2)
    }
    pub fn relative_to(&self, interval: f32) -> RelativePosF {
        RelativePosF(self.0 % interval, self.1 % interval, self.2 % interval)
    }
    pub fn tuple(&self) -> (f32, f32, f32) {
        (self.0, self.1, self.2)
    }
}
pub struct RelativePosF(f32, f32, f32);
