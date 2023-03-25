pub struct SpinButtonBuilder<'a, P: WindowMethods> {
    parent: `wxWindow*`,
    id: `wxWindowID`,
    pos: `const wxPoint&`,
    size: `const wxSize&`,
    style: `long`,
    name: `const wxString&`,
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
    pub fn parent(&mut self, parent: `wxWindow*`) -> &mut Self {
        self.parent = parent;
        self
    }
    pub fn id(&mut self, id: `wxWindowID`) -> &mut Self {
        self.id = id;
        self
    }
    pub fn pos(&mut self, pos: `const wxPoint&`) -> &mut Self {
        self.pos = pos;
        self
    }
    pub fn size(&mut self, size: `const wxSize&`) -> &mut Self {
        self.size = size;
        self
    }
    pub fn style(&mut self, style: `long`) -> &mut Self {
        self.style = style;
        self
    }
    pub fn name(&mut self, name: `const wxString&`) -> &mut Self {
        self.name = name;
        self
    }
    pub fn build(&mut self) -> SpinButton {
        let parent = self.parent.take().unwrap_or_else(|| `wxWindow*`::default());
        let id = self.id.take().unwrap_or_else(|| `wxWindowID`::default());
        let pos = self.pos.take().unwrap_or_else(|| `const wxPoint&`::default());
        let size = self.size.take().unwrap_or_else(|| `const wxSize&`::default());
        let style = self.style.take().unwrap_or_else(|| `long`::default());
        let name = self.name.take().unwrap_or_else(|| `const wxString&`::default());
        SpinButton::new(
            parent,
            id,
            pos,
            size,
            style,
            name,
        )
    }
}
