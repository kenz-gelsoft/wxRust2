pub struct SpinButtonBuilder<'a, P: WindowMethods> {
    parent: Option<Window>,
    id: c_int,
    pos: Point,
    size: Size,
    style: c_long,
    name: String,
}
impl<'a, P: WindowMethods> Buildable<'a, P, SpinButtonBuilder<'a, P>> for SpinButton {
    fn builder(parent: Option<&'a P>) -> SpinButtonBuilder<'a, P> {
        SpinButtonBuilder {
            parent: None,
            id: -1,
            pos: wxDefaultPosition,
            size: wxDefaultSize,
            style: wxSP_VERTICAL,
            name: "spinButton",
        }
    }
}
impl<'a, P: WindowMethods> SpinButtonBuilder<'a, P> {
    pub fn parent(&mut self, parent: Option<Window>) -> &mut Self {
        self.parent = parent;
        self
    }
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = pos;
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = size;
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    pub fn build(&mut self) -> SpinButton {
        SpinButton::new(
            self.parent,
            self.id,
            &self.pos,
            &self.size,
            self.style,
            &self.name,
        )
    }
}
