pub trait StartableService: std::any::Any + Sync + Send {
    fn build(db_state: &crate::database::state::StateDataBase) -> Self
    where
        Self: Sized;
}

pub struct ServiceRegistry {}

pub struct ServiceInstance {
    pub type_service: std::any::TypeId,
    pub builder: fn(
        db_state: &crate::database::state::StateDataBase,
    ) -> Box<dyn std::any::Any + Sync + Send>,
}
// TODO voir Any + Sync + Send pk il sont obligatoire
