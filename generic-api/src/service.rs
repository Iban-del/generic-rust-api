use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub trait StartableService: Any + Sync + Send {
    fn build(registry_manager: &crate::registry::RegistryManager) -> Self
    where
        Self: Sized;
}

pub struct ServiceRegistry {
    builders: HashMap<TypeId, fn(&crate::registry::RegistryManager) -> Box<dyn Any + Sync + Send>>,
    built: RwLock<HashMap<TypeId, Arc<dyn Any + Sync + Send>>>,
}

impl ServiceRegistry {
    /// Ne fait qu'enregistrer les recettes de construction (builders),
    /// sans exécuter aucune d'entre elles. Ne nécessite donc pas de
    /// RegistryManager déjà construit.
    pub fn new() -> Self {
        let mut builders = HashMap::new();
        for service in inventory::iter::<ServiceInstance> {
            builders.insert(service.type_service, service.builder);
        }

        Self {
            builders,
            built: RwLock::new(HashMap::new()),
        }
    }

    /// Construit (ou récupère si déjà construit) le service demandé.
    /// `registry_manager` est passé ici, au moment de l'appel, pas
    /// stocké dans le registre — ça évite la référence circulaire.
    pub fn get<T: StartableService>(
        &self,
        registry_manager: &crate::registry::RegistryManager,
    ) -> Arc<T> {
        if let Some(existing) = self.built.read().unwrap().get(&TypeId::of::<T>()) {
            return existing.clone().downcast::<T>().expect("type mismatch");
        }

        let builder = self
            .builders
            .get(&TypeId::of::<T>())
            .unwrap_or_else(|| panic!("Service {} non enregistré", std::any::type_name::<T>()));

        let instance: Arc<dyn Any + Sync + Send> = Arc::from(builder(registry_manager));
        self.built
            .write()
            .unwrap()
            .insert(TypeId::of::<T>(), instance.clone());

        instance.downcast::<T>().expect("type mismatch")
    }
}

pub struct ServiceInstance {
    pub type_service: TypeId,
    pub builder: fn(&crate::registry::RegistryManager) -> Box<dyn Any + Sync + Send>,
}

inventory::collect!(ServiceInstance);
