

impl ChildExt for super::Canvas {
    fn child(prt: &impl ContainerExt) -> Self {
        Self::new(prt)
    }
}
