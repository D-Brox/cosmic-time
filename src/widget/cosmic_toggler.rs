//! Show toggle controls using togglers.

use core::{
    alignment, layout, mouse, renderer, text,
    widget::{tree, Tree},
    Element, Layout, Length, Pixels, Rectangle, Size, Widget,
};
use cosmic::{
    iced::{core::Border, widget::toggler::Status},
    // widget::pane_grid::state,
};

use crate::{
    chain, id, lerp,
    reexports::{core, iced, widget},
};
pub use cosmic::iced::widget::toggler::{Catalog, Style};

/// A toggler widget.
#[allow(missing_debug_implementations)]
pub struct Toggler<'a, Message, Renderer>
where
    Renderer: text::Renderer,
{
    id: id::Toggler,
    is_toggled: bool,
    on_toggle: Box<dyn Fn(chain::Toggler, bool) -> Message + 'a>,
    label: Option<String>,
    width: Length,
    size: f32,
    text_size: Option<f32>,
    text_line_height: text::LineHeight,
    text_alignment: alignment::Horizontal,
    text_shaping: text::Shaping,
    spacing: f32,
    font: Option<Renderer::Font>,
    percent: f32,
    anim_multiplier: f32,
}

impl<'a, Message, Renderer> Toggler<'a, Message, Renderer>
where
    Renderer: text::Renderer,
{
    /// The default size of a [`Toggler`].
    pub const DEFAULT_SIZE: f32 = 24.0;

    /// Creates a new [`Toggler`].
    ///
    /// It expects:
    ///   * a boolean describing whether the [`Toggler`] is checked or not
    ///   * An optional label for the [`Toggler`]
    ///   * a function that will be called when the [`Toggler`] is toggled. It
    ///     will receive the new state of the [`Toggler`] and must produce a
    ///     `Message`.
    pub fn new<F>(id: id::Toggler, label: impl Into<Option<String>>, is_toggled: bool, f: F) -> Self
    where
        F: 'a + Fn(chain::Toggler, bool) -> Message,
    {
        Toggler {
            id,
            is_toggled,
            on_toggle: Box::new(f),
            label: label.into(),
            width: Length::Fill,
            size: Self::DEFAULT_SIZE,
            text_size: None,
            text_line_height: text::LineHeight::default(),
            text_alignment: alignment::Horizontal::Left,
            text_shaping: text::Shaping::Advanced,
            spacing: 0.0,
            font: None,
            percent: if is_toggled { 1.0 } else { 0.0 },
            anim_multiplier: 1.0,
        }
    }

    /// Sets the size of the [`Toggler`].
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into().0;
        self
    }

    /// Sets the width of the [`Toggler`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the text size o the [`Toggler`].
    pub fn text_size(mut self, text_size: impl Into<Pixels>) -> Self {
        self.text_size = Some(text_size.into().0);
        self
    }

    /// Sets the text [`LineHeight`] of the [`Toggler`].
    pub fn text_line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.text_line_height = line_height.into();
        self
    }

    /// Sets the horizontal alignment of the text of the [`Toggler`]
    pub fn text_alignment(mut self, alignment: alignment::Horizontal) -> Self {
        self.text_alignment = alignment;
        self
    }

    /// Sets the [`text::Shaping`] strategy of the [`Toggler`].
    pub fn text_shaping(mut self, shaping: text::Shaping) -> Self {
        self.text_shaping = shaping;
        self
    }

    /// Sets the spacing between the [`Toggler`] and the text.
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    /// Sets the [`Font`] of the text of the [`Toggler`]
    ///
    /// [`Font`]: cosmic::iced::text::Renderer::Font
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// The percent completion of the toggler animation.
    /// This is indented to automated cosmic-time use, and shouldn't
    /// need to be called manually.
    pub fn percent(mut self, percent: f32) -> Self {
        self.percent = percent;
        self
    }

    fn on_event(
        &mut self,
        _state: &mut Tree,
        event: &iced::Event,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn core::Clipboard,
        shell: &mut core::Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> iced::event::Status {
        match event {
            iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let mouse_over = cursor_position.is_over(layout.bounds());

                if mouse_over {
                    if self.is_toggled {
                        let off_animation =
                            chain::Toggler::off(self.id.clone(), self.anim_multiplier);
                        shell.publish((self.on_toggle)(off_animation, !self.is_toggled));
                    } else {
                        let on_animation =
                            chain::Toggler::on(self.id.clone(), self.anim_multiplier);
                        shell.publish((self.on_toggle)(on_animation, !self.is_toggled));
                    }

                    iced::event::Status::Captured
                } else {
                    iced::event::Status::Ignored
                }
            }
            _ => iced::event::Status::Ignored,
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, cosmic::Theme, Renderer>
    for Toggler<'a, Message, Renderer>
where
    Renderer: text::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, Length::Shrink)
    }

    fn state(&self) -> tree::State {
        tree::State::new(widget::text::State::<Renderer::Paragraph>::default())
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width);

        crate::utils::next_to_each_other(
            &limits,
            self.spacing,
            |limits| {
                if let Some(label) = self.label.as_deref() {
                    let state = tree
                        .state
                        .downcast_mut::<widget::text::State<Renderer::Paragraph>>();
                    let format = core::widget::text::Format {
                        width: self.width,
                        height: Length::Shrink,
                        size: self.text_size.map(iced::Pixels),
                        font: self.font,
                        line_height: self.text_line_height,
                        align_x: self.text_alignment.into(),
                        align_y: alignment::Vertical::Top,
                        shaping: self.text_shaping,
                        wrapping: text::Wrapping::default(),
                        ellipsize: text::Ellipsize::default(),
                    };
                    let node = core::widget::text::layout(state, renderer, limits, label, format);
                    match self.width {
                        Length::Fill => {
                            let size = node.size();
                            layout::Node::with_children(
                                Size::new(limits.width(Length::Fill).max().width, size.height),
                                vec![node],
                            )
                        }
                        _ => node,
                    }
                } else {
                    layout::Node::new(core::Size::ZERO)
                }
            },
            |_| layout::Node::new(Size::new(48., 24.)),
        )
    }

    fn update(
        &mut self,
        state: &mut Tree,
        event: &iced::Event,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn core::Clipboard,
        shell: &mut core::Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        _ = self.on_event(
            state,
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor_position.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &cosmic::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let mut children = layout.children();

        if let Some(_label) = &self.label {
            let label_layout = children.next().unwrap();
            let state: &widget::text::State<Renderer::Paragraph> = tree.state.downcast_ref();
            widget::text::draw(
                renderer,
                style,
                label_layout.bounds(),
                state.paragraph.raw(),
                widget::text::Style::default(),
                viewport,
            );
        }

        let toggler_layout = children.next().unwrap();
        let bounds = toggler_layout.bounds();

        let is_mouse_over = cursor_position.is_over(bounds);

        let style = blend_appearances(
            theme.style(
                &(),
                if is_mouse_over {
                    Status::Hovered { is_toggled: false }
                } else {
                    Status::Active { is_toggled: false }
                },
            ),
            theme.style(
                &(),
                if is_mouse_over {
                    Status::Hovered { is_toggled: true }
                } else {
                    Status::Active { is_toggled: true }
                },
            ),
            self.percent,
        );

        let space = style.handle_margin;

        let toggler_background_bounds = Rectangle {
            x: bounds.x,
            y: bounds.y,
            width: bounds.width,
            height: bounds.height,
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: toggler_background_bounds,
                border: Border {
                    radius: style.border_radius,
                    ..Default::default()
                },
                ..renderer::Quad::default()
            },
            style.background,
        );

        let toggler_foreground_bounds = Rectangle {
            x: bounds.x
                + lerp(
                    space,
                    bounds.width - space - (bounds.height - (2.0 * space)),
                    self.percent,
                ),

            y: bounds.y + space,
            width: bounds.height - (2.0 * space),
            height: bounds.height - (2.0 * space),
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: toggler_foreground_bounds,
                border: Border {
                    radius: style.handle_radius,
                    ..Default::default()
                },
                ..renderer::Quad::default()
            },
            style.foreground,
        );
    }
}

impl<'a, Message, Renderer> From<Toggler<'a, Message, Renderer>>
    for Element<'a, Message, cosmic::Theme, Renderer>
where
    Message: 'a,
    Renderer: 'a + text::Renderer,
{
    fn from(
        toggler: Toggler<'a, Message, Renderer>,
    ) -> Element<'a, Message, cosmic::Theme, Renderer> {
        Element::new(toggler)
    }
}

fn blend_appearances(first: Style, mut other: Style, percent: f32) -> Style {
    if percent == 0. {
        first
    } else if percent == 1. {
        other
    } else {
        other.background = match (first.background, other.background) {
            (iced::Background::Color(first_color), iced::Background::Color(mut other_color)) => {
                let other_background = std::mem::take(&mut other_color).into_linear();
                iced::Background::Color(
                    crate::utils::static_array_from_iter::<f32, 4>(
                        first_color
                            .into_linear()
                            .iter()
                            .zip(other_background.iter())
                            .map(|(o, t)| o * (1.0 - percent) + t * percent),
                    )
                    .into(),
                )
            }
            _ => todo!("I don't really now what to do here"),
        };

        other
    }
}
