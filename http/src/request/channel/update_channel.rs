use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    channel::{permission_overwrite::PermissionOverwrite, Channel, ChannelType},
    id::ChannelId,
};

#[derive(Clone, Debug)]
/// Returned when the channel can not be updated as configured.
pub enum UpdateChannelError {
    /// The length of the name is either fewer than 2 UTF-16 characters or
    /// more than 100 UTF-16 characters.
    NameInvalid,
    /// The seconds of the rate limit per user is more than 21600.
    RateLimitPerUserInvalid,
    /// The length of the topic is more than 1024 UTF-16 characters.
    TopicInvalid,
}

impl Display for UpdateChannelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NameInvalid => f.write_str("the length of the name is invalid"),
            Self::RateLimitPerUserInvalid => f.write_str("the rate limit per user is invalid"),
            Self::TopicInvalid => f.write_str("the topic is invalid"),
        }
    }
}

impl Error for UpdateChannelError {}

// The Discord API doesn't require the `name` and `kind` fields to be present,
// but it does require them to be non-null.
#[derive(Default, Serialize)]
struct UpdateChannelFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    bitrate: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nsfw: Option<bool>,
    #[allow(clippy::option_option)]
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<Option<ChannelId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission_overwrites: Option<Vec<PermissionOverwrite>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_per_user: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_limit: Option<u64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<ChannelType>,
}

/// Update a channel.
///
/// All fields are optional. The minimum length of the name is 2 UTF-16 characters and the maximum
/// is 100 UTF-16 characters.
///
/// # Errors
///
/// Returns a [`UpdateChannelError::NameInvalid`] when the length of the name is either fewer than
/// 2 UTF-16 characters or more than 100 UTF-16 characters.
///
/// Returns a [`UpdateChannelError::RateLimitPerUserInvalid`] when the seconds of the rate limit per
/// user is more than 21600.
///
/// Returns a [`UpdateChannelError::TopicInvalid`] when the length of the topic is more than
/// 1024 UTF-16 characters.
///
/// [`UpdateChannelError::NameInvalid`]: ../enum.UpdateChannelError.html#variant.NameInvalid
/// [`UpdateChannelError::RateLimitPerUserInvalid`]: ../enum.UpdateChannelError.html#variant.RateLimitPerUserInvalid
/// [`UpdateChannelError::TopicInvalid`]: ../enum.UpdateChannelError.html#variant.TopicInvalid
pub struct UpdateChannel<'a> {
    channel_id: ChannelId,
    fields: UpdateChannelFields,
    fut: Option<Pending<'a, Channel>>,
    http: &'a Client,
    reason: Option<String>,
}

impl<'a> UpdateChannel<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            fields: UpdateChannelFields::default(),
            fut: None,
            http,
            reason: None,
        }
    }

    /// Set the bitrate of the channel. Applicable to voice channels only.
    pub fn bitrate(mut self, bitrate: u64) -> Self {
        self.fields.bitrate.replace(bitrate);

        self
    }

    /// Set the name.
    ///
    /// The minimum length is 2 UTF-16 characters and the maximum is 100 UTF-16
    /// characters.
    ///
    /// # Errors
    ///
    /// Returns [`UpdateChannelError::NameInvalid`] if the name length is
    /// too short or too long.
    ///
    /// [`UpdateChannelError::NameInvalid`]: enum.UpdateChannelError.html#variant.NameInvalid
    pub fn name(self, name: impl Into<String>) -> Result<Self, UpdateChannelError> {
        self._name(name.into())
    }

    fn _name(mut self, name: String) -> Result<Self, UpdateChannelError> {
        if !validate::channel_name(&name) {
            return Err(UpdateChannelError::NameInvalid);
        }

        self.fields.name.replace(name);

        Ok(self)
    }

    /// Set whether the channel is marked as NSFW.
    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.fields.nsfw.replace(nsfw);

        self
    }

    /// If this is specified, and the parent ID is a `ChannelType::CategoryChannel`, move this
    /// channel to a child of the category channel.
    pub fn parent_id(mut self, parent_id: impl Into<Option<ChannelId>>) -> Self {
        self.fields.parent_id.replace(parent_id.into());

        self
    }

    /// Set the permission overwrites of a channel. This will overwrite all permissions that the
    /// channel currently has, so use with caution!
    pub fn permission_overwrites(
        mut self,
        permission_overwrites: Vec<PermissionOverwrite>,
    ) -> Self {
        self.fields
            .permission_overwrites
            .replace(permission_overwrites);

        self
    }

    /// Set the position of the channel.
    ///
    /// Positions are numerical and zero-indexed. If you place a channel at position 2, channels
    /// 2-n will shift down one position and the initial channel will take its place.
    pub fn position(mut self, position: u64) -> Self {
        self.fields.position.replace(position);

        self
    }

    /// Set the number of seconds that a user must wait before before they are able to send another
    /// message.
    ///
    /// The minimum is 0 and the maximum is 21600. Refer to [the discord docs] for more details.
    /// This is also known as "Slow Mode".
    ///
    /// # Errors
    ///
    /// Returns [`GetGuildPruneCountError::RateLimitPerUserInvalid`] if the amount is greater than
    /// 21600.
    ///
    /// [`GetGuildPruneCountError::RateLimitPerUserInvalid`]: enum.GetGuildPruneCountError.html#variant.RateLimitPerUserInvalid
    /// [the discord docs]: https://discordapp.com/developers/docs/resources/channel#channel-object-channel-structure>
    pub fn rate_limit_per_user(
        mut self,
        rate_limit_per_user: u64,
    ) -> Result<Self, UpdateChannelError> {
        if rate_limit_per_user > 21600 {
            return Err(UpdateChannelError::RateLimitPerUserInvalid);
        }

        self.fields.rate_limit_per_user.replace(rate_limit_per_user);

        Ok(self)
    }

    /// Set the topic.
    ///
    /// The maximum length is 1024 UTF-16 characters. Refer to [the discord docs] for more details.
    ///
    /// # Errors
    ///
    /// Returns [`CreateGuildChannel::TopicInvalid`] if the topic length is
    /// too long.
    ///
    /// [the discord docs]: https://discordapp.com/developers/docs/resources/channel#channel-object-channel-structure
    /// [`CreateGuildChannel::TopicInvalid`]: enum.CreateGuildChannel.html#variant.TopicInvalid
    pub fn topic(self, topic: impl Into<String>) -> Result<Self, UpdateChannelError> {
        self._topic(topic.into())
    }

    fn _topic(mut self, topic: String) -> Result<Self, UpdateChannelError> {
        if topic.chars().count() > 1024 {
            return Err(UpdateChannelError::TopicInvalid);
        }

        self.fields.topic.replace(topic);

        Ok(self)
    }

    /// For voice channels, set the user limit.
    ///
    /// Set to 0 for no limit. Limit can otherwise be between 1 and 99 inclusive. Refer to [the
    /// discord docs] for more details.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/channel#modify-channel-json-params
    pub fn user_limit(mut self, user_limit: u64) -> Self {
        self.fields.user_limit.replace(user_limit);

        self
    }

    /// Set the kind of channel.
    ///
    /// Only conversion between `ChannelType::GuildText` and `ChannelType::GuildNews` is possible,
    /// and only if the guild has the `NEWS` feature enabled. Refer to [the discord docs] for more
    /// details.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/channel#modify-channel-json-params
    pub fn kind(mut self, kind: ChannelType) -> Self {
        self.fields.kind.replace(kind);

        self
    }

    /// Attach an audit log reason to this request.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason.replace(reason.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                crate::json_to_vec(&self.fields)?,
                headers,
                Route::UpdateChannel {
                    channel_id: self.channel_id.0,
                },
            ))
        } else {
            Request::from((
                crate::json_to_vec(&self.fields)?,
                Route::UpdateChannel {
                    channel_id: self.channel_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(UpdateChannel<'_>, Channel);
