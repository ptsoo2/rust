use std::fmt;

use lapin::options::BasicPublishOptions;

pub struct Message {
    pub app_id_: String,
    pub body_: String,
    pub exchange_: String,
    pub routing_key_: String,
    pub channel_no_: u16,
    pub basic_publish_options_: BasicPublishOptions,
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Message")
            .field("app_id", &self.app_id_)
            .field("body", &self.body_)
            .field("exchange", &self.exchange_)
            .field("routing_key", &self.routing_key_)
            .field("channel_no", &self.channel_no_)
            .field("basic_publish_options", &self.basic_publish_options_)
            .finish()
    }
}

impl Clone for Message {
    fn clone(&self) -> Self {
        Self {
            app_id_: self.app_id_.clone(),
            body_: self.body_.clone(),
            exchange_: self.exchange_.clone(),
            routing_key_: self.routing_key_.clone(),
            channel_no_: self.channel_no_,
            basic_publish_options_: self.basic_publish_options_.clone(),
        }
    }
}
