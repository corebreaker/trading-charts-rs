//! Mounting of the chart children in document order.
//!
//! Yew creates the sibling components from the last to the first, so their first renders and effects do not follow
//! the document order. But the order matters to the chart: the panes are allocated in the order of the panels, and
//! the series added last are drawn above the others.
//!
//! The panels, series and watermarks thus render an invisible marker element, and queue their mount on the
//! [`MountQueue`] of the chart from their mount effect. The queue runs the pending mounts in a microtask, once all the
//! effects of the render are done, sorted by the document position of their markers.

use crate::PanelId;

use std::{
    cell::{Cell, RefCell},
    cmp::Ordering,
    rc::Rc,
};
use web_sys::Node;
use yew::prelude::*;

/// Mount of a chart child, which runs once its marker is placed among the others.
type PendingMount = (Option<Node>, Box<dyn FnOnce()>);

#[derive(Default)]
struct MountQueueInner {
    pending:   RefCell<Vec<PendingMount>>,
    scheduled: Cell<bool>,
}

/// Queue of the mounts of the children of a chart, run in document order. It is provided by the `Chart` component.
#[derive(Clone, Default)]
pub(crate) struct MountQueue(Rc<MountQueueInner>);

impl PartialEq for MountQueue {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

fn document_order(a: &Option<Node>, b: &Option<Node>) -> Ordering {
    match (a, b) {
        (Some(a), Some(b)) if a.is_same_node(Some(b)) => Ordering::Equal,
        (Some(a), Some(b)) if a.compare_document_position(b) & Node::DOCUMENT_POSITION_FOLLOWING != 0 => Ordering::Less,
        (Some(_), Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

impl MountQueue {
    /// Queue a mount, placed by the marker element.
    ///
    /// The mounts queued for the same marker run in the order they were queued.
    pub(crate) fn push(&self, marker: &NodeRef, mount: impl FnOnce() + 'static) {
        self.0.pending.borrow_mut().push((marker.get(), Box::new(mount)));

        if !self.0.scheduled.replace(true) {
            let queue = self.clone();

            yew::platform::spawn_local(async move { queue.flush() });
        }
    }

    fn flush(&self) {
        self.0.scheduled.set(false);

        let mut pending = std::mem::take(&mut *self.0.pending.borrow_mut());

        // The sort is stable: the mounts of the same marker keep their order.
        pending.sort_by(|(a, _), (b, _)| document_order(a, b));
        for (_, mount) in pending {
            mount();
        }
    }
}

/// The mount queue of the enclosing `Chart`.
///
/// # Panics
///
/// Panics when called outside of a `Chart`.
#[hook]
pub(crate) fn use_mount_queue() -> MountQueue {
    use_context::<MountQueue>().expect("Chart context not found. Ensure the component is rendered inside <Chart>.")
}

/// Pane of a `ChartPanel`, allocated when the panel is mounted, provided to its children.
#[derive(Clone, Default)]
pub(crate) struct PanelSlot(Rc<Cell<Option<PanelId>>>);

impl PanelSlot {
    pub(crate) fn get(&self) -> Option<PanelId> {
        self.0.get()
    }

    pub(crate) fn set(&self, panel: PanelId) {
        self.0.set(Some(panel));
    }
}

impl PartialEq for PanelSlot {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

/// The pane of the enclosing `ChartPanel`: `None` outside of a panel, or before the panel is mounted.
#[hook]
pub(crate) fn use_panel() -> Option<PanelSlot> {
    use_context::<PanelSlot>()
}

/// Invisible element placing a chart child in the document.
pub(crate) fn marker(node: &NodeRef) -> Html {
    html! { <span ref={node.clone()} style="display:none"></span> }
}

/// Flag telling whether a component is still mounted, for the mounts which run after the component is dropped.
#[derive(Clone)]
pub(crate) struct Alive(Rc<Cell<bool>>);

impl Alive {
    pub(crate) fn new() -> Self {
        Self(Rc::new(Cell::new(true)))
    }

    pub(crate) fn get(&self) -> bool {
        self.0.get()
    }

    pub(crate) fn drop_component(&self) {
        self.0.set(false);
    }
}
