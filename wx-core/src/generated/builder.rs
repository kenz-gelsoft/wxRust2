use super::*;

pub struct SpinButtonBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
    name: String,
}
impl<'a, P: WindowMethods> Buildable<'a, P, SpinButtonBuilder<'a, P>> for SpinButton {
    fn builder(parent: Option<&'a P>) -> SpinButtonBuilder<'a, P> {
        SpinButtonBuilder {
            parent: parent,
            id: -1,
            pos: None,
            size: None,
            style: SP_VERTICAL.into(),
            name: "spinButton".to_owned(),
        }
    }
}
impl<'a, P: WindowMethods> SpinButtonBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = name.to_owned();
        self
    }
    pub fn build(&mut self) -> SpinButton {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        SpinButton::new(
            self.parent,
            self.id,
            &pos,
            &size,
            self.style,
            &self.name,
        )
    }
}
