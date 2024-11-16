use leptos::{
    prelude::{
        guards::{Derefable, ReadGuard},
        DefinedAt, ReadUntracked, RwSignal, Set, Track,
    },
    tachys::{html::node_ref::NodeRefContainer, renderer::types::Element},
};
use send_wrapper::SendWrapper;

/// A reactive reference to a DOM node that can be used with the `node_ref` attribute.
#[derive(Debug)]
pub struct GenericNodeRef(RwSignal<Option<SendWrapper<Element>>>);

impl GenericNodeRef {
    /// Creates a new node reference.
    #[track_caller]
    pub fn new() -> Self {
        Self(RwSignal::new(None))
    }
}

impl Default for GenericNodeRef {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for GenericNodeRef {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for GenericNodeRef {}

impl DefinedAt for GenericNodeRef {
    fn defined_at(&self) -> Option<&'static std::panic::Location<'static>> {
        self.0.defined_at()
    }
}

impl ReadUntracked for GenericNodeRef {
    type Value = ReadGuard<Option<Element>, Derefable<Option<Element>>>;

    fn try_read_untracked(&self) -> Option<Self::Value> {
        Some(ReadGuard::new(Derefable(
            self.0.try_read_untracked()?.as_deref().cloned(),
        )))
    }
}

impl Track for GenericNodeRef {
    fn track(&self) {
        self.0.track();
    }
}

macro_rules! impl_generic_node_ref {
    ($($element:ident),*,) => {
        $(impl NodeRefContainer<leptos::html::$element> for GenericNodeRef {
            fn load(self, el: &Element) {
                // safe to construct SendWrapper here, because it will only run in the browser
                // so it will always be accessed or dropped from the main thread
                self.0.set(Some(SendWrapper::new(el.clone())));
            }
        })*
    };
}

impl_generic_node_ref!(
    A, Abbr, Address, Area, Article, Aside, Audio, B, Base, Bdi, Bdo, Blockquote, Body, Br, Button,
    Canvas, Caption, Cite, Code, Col, Colgroup, Data, Datalist, Dd, Del, Details, Dfn, Dialog, Div,
    Dl, Dt, Em, Embed, Fieldset, Figcaption, Figure, Footer, Form, H1, H2, H3, H4, H5, H6, Head,
    Header, Hgroup, Hr, Html, I, Iframe, Img, Input, Ins, Kbd, Label, Legend, Li, Link, Main, Map,
    Mark, Menu, Meta, Meter, Nav, Noscript, Object, Ol, Optgroup, Option_, Output, P, Picture,
    Portal, Pre, Progress, Q, Rp, Rt, Ruby, S, Samp, Script, Search, Section, Select, Slot, Small,
    Source, Span, Strong, Style, Sub, Summary, Sup, Table, Tbody, Td, Template, Textarea, Tfoot,
    Th, Thead, Time, Title, Tr, Track, U, Ul, Var, Video, Wbr,
);
