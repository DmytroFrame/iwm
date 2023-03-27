use crate::game::shared_values::CHUNK_OWNER;

pub(crate) async fn set_chunk_owner((x, z): (i32, i32), id: i32) {
    CHUNK_OWNER.lock().await.entry((x, z)).or_insert(id);
}

pub(crate) async fn get_chunk_owner((x, z): (i32, i32)) -> Option<i32> {
    CHUNK_OWNER.lock().await.get(&(x, z)).cloned()
}
