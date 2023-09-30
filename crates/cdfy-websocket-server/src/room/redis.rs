use super::{Room, RoomStore};
use anyhow::{Error, Result};
use redis::{Commands, Connection};

pub struct RedisRoomStore {
    redis_addr: String,
}

static REDIS_ADDR: &str = "redis://127.0.0.1";

impl Default for RedisRoomStore {
    fn default() -> Self {
        Self {
            redis_addr: REDIS_ADDR.to_string(),
        }
    }
}

impl RedisRoomStore {
    fn conn(&self) -> Result<Connection> {
        let client = redis::Client::open(self.redis_addr.as_ref()).map_err(Error::from)?;
        client.get_connection().map_err(Error::from)
    }
}

impl RoomStore for RedisRoomStore {
    fn list_ids(&self) -> Result<Vec<String>> {
        let mut con = self.conn()?;
        Ok(con
            .scan_match("rooms:*")
            .map_err(Error::from)?
            .into_iter()
            .collect::<Vec<String>>())
    }

    fn set(&self, room: &Room) -> Result<()> {
        let mut con = self.conn()?;

        let key = format!("rooms:{}", room.room_id);
        let room_json = serde_json::to_string(room).unwrap();
        con.set(&key, room_json).map_err(Error::from)
    }

    fn get(&self, room_id: String) -> Result<Room> {
        let mut con = self.conn()?;
        let key = format!("rooms:{}", room_id);
        let room: String = con.get(&key).map_err(Error::from)?;
        let room: Room = serde_json::from_str(&room).map_err(Error::from)?;
        Ok(room)
    }

    fn delete(&self, room_id: String) -> Result<()> {
        let mut con = self.conn()?;
        let key = format!("rooms:{}", room_id);
        con.del(&key).map_err(Error::from)?;
        Ok(())
    }
}
