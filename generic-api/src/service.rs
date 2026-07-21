use std::collections::HashMap;

pub trait StartableService: std::any::Any + Sync + Send {
    fn build(db_state: &crate::database::state::StateDataBase) -> Self
    where
        Self: Sized;
}

pub struct ServiceRegistry {
    registers: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any + Sync + Send>>,
}

impl ServiceRegistry
where
    dyn StartableService: std::any::Any,
{
    pub fn new() -> Self {
        Self {
            registers: HashMap::new(),
        }
    }

    pub fn register(&mut self, service: Box<dyn StartableService>) {
        if !self.registers.contains_key(&service.type_id()) {
            self.registers.insert(service.type_id(), service);
        }
    }

    pub fn get<T: StartableService>(&self) -> Option<&T> {
        let val = match self.registers.get(&std::any::TypeId::of::<T>()) {
            Some(service) => service.downcast_ref::<T>(),
            None => None,
        };
        val
    }
}

pub struct ServiceInstance {
    pub type_service: std::any::TypeId,
    pub builder: fn(
        db_state: &crate::database::state::StateDataBase,
    ) -> Box<dyn std::any::Any + Sync + Send>,
}

inventory::collect!(ServiceInstance);
