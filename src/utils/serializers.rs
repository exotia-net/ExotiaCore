use serde::{Serialize, Serializer, ser::SerializeStruct};

use crate::entities::{users, survival_economy, wallet, calendars};

impl Serialize for calendars::Model {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        let mut state = serializer.serialize_struct("calendars", 7)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("userId", &self.user_id)?;
        state.serialize_field("step", &self.step)?;
        state.serialize_field("streak", &self.streak)?;
        state.serialize_field("lastObtained", &self.last_obtained)?;
        let obtained_rewards = if self.obtained_rewards.len() == 0 {
            Vec::new()
        } else {
            self.obtained_rewards.split("|").map(|v| v.parse::<i32>().unwrap_or(0)).collect::<Vec<_>>()
        };
        state.serialize_field("obtainedRewards", &obtained_rewards)?;
        state.serialize_field("createdAt", &self.created_at)?;
        state.serialize_field("updatedAt", &self.updated_at)?;
        state.end()
    }
}

impl Serialize for survival_economy::Model {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("users", 5)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("userId", &self.user_id)?;
        state.serialize_field("balance", &self.balance)?;
        state.serialize_field("updatedAt", &self.updated_at)?;
        state.serialize_field("createdAt", &self.created_at)?;
        state.end()
    }
}

impl Serialize for users::Model {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("users", 6)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("uuid", &self.uuid)?;
        state.serialize_field("firstIp", &self.first_ip)?;
        state.serialize_field("lastIp", &self.last_ip)?;
        state.serialize_field("updatedAt", &self.updated_at)?;
        state.serialize_field("createdAt", &self.created_at)?;
        state.end()
    }
}

impl Serialize for wallet::Model {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("wallet", 6)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("userId", &self.user_id)?;
        state.serialize_field("coins", &self.coins)?;
        state.serialize_field("spentCoins", &self.spent_coins)?;
        state.serialize_field("updatedAt", &self.updated_at)?;
        state.serialize_field("createdAt", &self.created_at)?;
        state.end()
    }
}
