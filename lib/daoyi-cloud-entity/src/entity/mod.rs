pub mod system;

pub trait BaseEntityTrait {
    fn set_creator(&mut self, u: String);
    fn set_updater(&mut self, u: String);
    fn set_tenant_id(&mut self, _t: i64) {}
}
