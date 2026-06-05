use uuid::Uuid;

use crate::application::SaveFoo;
use crate::domain::Foo;


pub async fn create_foo<T: SaveFoo>(tx: &mut T, bar_count: u16) {
    let id = Uuid::new_v4();
    let foo = Foo::new(id, bar_count);
    tx.save_foo(foo).await;
}
