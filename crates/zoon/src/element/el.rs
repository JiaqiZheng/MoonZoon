use crate::*;
use std::{iter, marker::PhantomData};

// ------ ------
//   Element
// ------ ------

make_flags!(Child);

type ElRawEl = RawHtmlEl<web_sys::HtmlElement>;

pub struct El<ChildFlag> {
    raw_el: ElRawEl,
    flags: PhantomData<ChildFlag>,
}

impl El<ChildFlagNotSet> {
    pub fn new() -> Self {
        Self::with_tag(Tag::Custom("div"))
    }
}

impl<ChildFlag> Element for El<ChildFlag> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<ChildFlag> IntoIterator for El<ChildFlag> {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

impl<ChildFlag> UpdateRawEl<ElRawEl> for El<ChildFlag> {
    fn update_raw_el(mut self, updater: impl FnOnce(ElRawEl) -> ElRawEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl ChoosableTag for El<ChildFlagNotSet> {
    fn with_tag(tag: Tag) -> Self {
        run_once!(|| {
            global_styles()
                .style_group(StyleGroup::new(".el > .center_x").style("align-self", "center"))
                .style_group(
                    StyleGroup::new(".el > .center_y")
                        .style("margin-top", "auto")
                        .style("margin-bottom", "auto"),
                )
                .style_group(StyleGroup::new(".el > .align_top").style("margin-bottom", "auto"))
                .style_group(StyleGroup::new(".el > .align_bottom").style("margin-top", "auto"))
                .style_group(StyleGroup::new(".el > .align_left").style("align-self", "flex-start"))
                .style_group(StyleGroup::new(".el > .align_right").style("align-self", "flex-end"))
                .style_group(StyleGroup::new(".el > .exact_height").style("flex-shrink", "0"))
                .style_group(StyleGroup::new(".el > .fill_height").style("flex-grow", "1"));
        });
        Self {
            raw_el: RawHtmlEl::new(tag.as_str())
                .class("el")
                .style("display", "inline-flex")
                .style("flex-direction", "column"),
            flags: PhantomData,
        }
    }
}
impl<ChildFlag> Styleable<'_, ElRawEl> for El<ChildFlag> {}
impl<ChildFlag> KeyboardEventAware<ElRawEl> for El<ChildFlag> {}
impl<ChildFlag> MouseEventAware<ElRawEl> for El<ChildFlag> {}
impl<ChildFlag> PointerEventAware<ElRawEl> for El<ChildFlag> {}
impl<ChildFlag> TouchEventAware<ElRawEl> for El<ChildFlag> {}
impl<ChildFlag> MutableViewport<ElRawEl> for El<ChildFlag> {}
impl<ChildFlag> ResizableViewport<ElRawEl> for El<ChildFlag> {}
impl<ChildFlag> Hookable<ElRawEl> for El<ChildFlag> {
}
impl<ChildFlag> AddNearbyElement<'_, ElRawEl> for El<ChildFlag> {}
impl<ChildFlag> HasClassId<ElRawEl> for El<ChildFlag> {}
impl<ChildFlag> SelectableTextContent<ElRawEl> for El<ChildFlag> {}

// ------ ------
//  Attributes
// ------ ------

impl<'a, ChildFlag> El<ChildFlag> {
    pub fn child(mut self, child: impl IntoOptionElement<'a> + 'a) -> El<ChildFlagSet>
    where
        ChildFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.child(child);
        self.into_type()
    }

    pub fn child_signal(
        mut self,
        child: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> El<ChildFlagSet>
    where
        ChildFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.child_signal(child);
        self.into_type()
    }

    fn into_type<NewChildFlag>(self) -> El<NewChildFlag> {
        El {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
