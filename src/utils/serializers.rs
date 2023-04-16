use serde::{Serialize, Serializer, ser::SerializeStruct};

use crate::entities::users;

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