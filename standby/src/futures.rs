use futures_channel::{
    mpsc::UnboundedReceiver as MpscReceiver,
    oneshot::{Canceled, Receiver},
};
use futures_util::{
    future::FutureExt,
    stream::{Stream, StreamExt},
};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use twilight_model::gateway::{
    event::Event,
    payload::{MessageCreate, ReactionAdd},
};

/// The future returned from [`Standby::wait_for_event`].
///
/// [`Standby::wait_for_event`]: struct.Standby.html#method.wait_for_event
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct WaitForEventFuture {
    pub(crate) rx: Receiver<Event>,
}

impl Future for WaitForEventFuture {
    type Output = Result<Event, Canceled>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.rx.poll_unpin(cx)
    }
}

/// The stream returned from [`Standby::wait_for_event_stream`].
///
/// [`Standby::wait_for_event_stream`]: struct.Standby.html#method.wait_for_event_stream
#[derive(Debug)]
#[must_use = "streams do nothing unless you poll them"]
pub struct WaitForEventStream {
    pub(crate) rx: MpscReceiver<Event>,
}

impl Stream for WaitForEventStream {
    type Item = Event;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_next_unpin(cx)
    }
}

/// The future returned from [`Standby::wait_for`].
///
/// [`Standby::wait_for`]: struct.Standby.html#method.wait_for
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct WaitForGuildEventFuture {
    pub(crate) rx: Receiver<Event>,
}

impl Future for WaitForGuildEventFuture {
    type Output = Result<Event, Canceled>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.rx.poll_unpin(cx)
    }
}

/// The stream returned from [`Standby::wait_for_guild_event_stream`].
///
/// [`Standby::wait_for_guild_event_stream`]: struct.Standby.html#method.wait_for_guild_event_stream
#[derive(Debug)]
#[must_use = "streams do nothing unless you poll them"]
pub struct WaitForGuildEventStream {
    pub(crate) rx: MpscReceiver<Event>,
}

impl Stream for WaitForGuildEventStream {
    type Item = Event;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_next_unpin(cx)
    }
}

/// The future returned from [`Standby::wait_for_message`].
///
/// [`Standby::wait_for_message`]: struct.Standby.html#method.wait_for_message
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct WaitForMessageFuture {
    pub(crate) rx: Receiver<MessageCreate>,
}

impl Future for WaitForMessageFuture {
    type Output = Result<MessageCreate, Canceled>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.rx.poll_unpin(cx)
    }
}

/// The stream returned from [`Standby::wait_for_message_stream`].
///
/// [`Standby::wait_for_message_stream`]: struct.Standby.html#method.wait_for_message_stream
#[derive(Debug)]
#[must_use = "streams do nothing unless you poll them"]
pub struct WaitForMessageStream {
    pub(crate) rx: MpscReceiver<MessageCreate>,
}

impl Stream for WaitForMessageStream {
    type Item = MessageCreate;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_next_unpin(cx)
    }
}

/// The future returned from [`Standby::wait_for_reaction`].
///
/// [`Standby::wait_for_reaction`]: struct.Standby.html#method.wait_for_reaction
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct WaitForReactionFuture {
    pub(crate) rx: Receiver<ReactionAdd>,
}

impl Future for WaitForReactionFuture {
    type Output = Result<ReactionAdd, Canceled>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.rx.poll_unpin(cx)
    }
}

/// The stream returned from [`Standby::wait_for_reaction_stream`].
///
/// [`Standby::wait_for_reaction_stream`]: struct.Standby.html#method.wait_for_reaction_stream
#[derive(Debug)]
#[must_use = "streams do nothing unless you poll them"]
pub struct WaitForReactionStream {
    pub(crate) rx: MpscReceiver<ReactionAdd>,
}

impl Stream for WaitForReactionStream {
    type Item = ReactionAdd;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_next_unpin(cx)
    }
}
