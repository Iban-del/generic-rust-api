/// Abstraction générique représentant une connexion à une source de données.
///
/// Ce trait définit le contrat commun que doit respecter toute implémentation
/// de connexion (SQL, NoSQL, etc.), permettant d'établir une connexion et
/// d'obtenir un objet capable d'exécuter des requêtes.
#[async_trait::async_trait]
pub trait Connection {
    /// Établit la connexion à la source de données.
    ///
    /// Cette méthode est asynchrone et mute l'état interne de la connexion
    /// (par exemple en initialisant un pool de connexions).
    ///
    /// # Erreurs
    ///
    /// Retourne une [`crate::error::DatabaseError`] si la connexion échoue.
    async fn connect(&mut self) -> Result<(), crate::error::DatabaseError>;

    /// Fournit un objet permettant de construire et d'exécuter des requêtes
    /// sur la connexion courante.
    ///
    /// # Erreurs
    ///
    /// Retourne une [`crate::error::DatabaseError`] si l'objet de requête
    /// ne peut pas être créé.
    fn get_query(
        &self,
    ) -> Result<Box<dyn crate::database::query::Query>, crate::error::DatabaseError>;
}

// ---- gestion des connexions aux base sql ----

/// Implémentation de [`Connection`] pour les bases de données SQL,
/// basée sur un pool de connexions générique `sqlx::AnyPool`.
///
/// Cette structure conserve les paramètres nécessaires à l'établissement
/// du pool (URL de connexion, bornes min/max du nombre de connexions)
/// ainsi que le pool lui-même une fois la connexion établie.
pub struct SqlConnection<DB: sqlx::Database> {
    /// Pool de connexions SQL actif, initialisé après un appel réussi à
    /// [`SqlConnection::connect`]. Vaut `None` tant que la connexion
    /// n'a pas été établie.
    pool: Option<sqlx::Pool<DB>>,
    /// URL de connexion à la base de données (chaîne de connexion).
    url: String,
    /// Nombre maximum de connexions autorisées dans le pool.
    max_connections: u32,
    /// Nombre minimum de connexions maintenues dans le pool.
    min_connections: u32,
}

impl<DB> SqlConnection<DB>
where
    DB: sqlx::Database,
{
    /// Crée une nouvelle instance de [`SqlConnection`] sans établir de connexion.
    ///
    /// Le pool n'est initialisé qu'après un appel à [`SqlConnection::connect`].
    ///
    /// # Paramètres
    ///
    /// * `url` - Chaîne de connexion vers la base de données.
    /// * `max_connections` - Nombre maximum de connexions dans le pool.
    /// * `min_connections` - Nombre minimum de connexions dans le pool.
    pub fn new(url: String, max_connections: u32, min_connections: u32) -> Self {
        Self {
            pool: None,
            url: url,
            max_connections: max_connections,
            min_connections: min_connections,
        }
    }
}

#[async_trait::async_trait]
impl<DB> Connection for SqlConnection<DB>
where
    DB: sqlx::Database,
{
    /// Initialise le pool de connexions SQL en utilisant les paramètres
    /// configurés (`url`, `max_connections`, `min_connections`).
    ///
    /// Une fois la connexion établie avec succès, le pool est stocké
    /// dans `self.pool`.
    ///
    /// # Erreurs
    ///
    /// Retourne une [`crate::error::DatabaseError`] si l'établissement
    /// du pool de connexions échoue.
    async fn connect(&mut self) -> Result<(), crate::error::DatabaseError> {
        let pool = sqlx::pool::PoolOptions::<DB>::new()
            .max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .connect(&self.url)
            .await
            .map_err(|e| crate::error::DatabaseError::Sql(crate::error::SqlError::Sqlx(e)))?;

        self.pool = Some(pool);
        Ok(())
    }

    /// Retourne un objet [`crate::database::query::SqlQuery`] permettant
    /// de construire et d'exécuter des requêtes SQL sur cette connexion.
    ///
    /// # Erreurs
    ///
    /// Retourne une [`crate::error::DatabaseError`] en cas d'échec de
    /// création de l'objet de requête.
    fn get_query(
        &self,
    ) -> Result<Box<dyn crate::database::query::Query>, crate::error::DatabaseError> {
        let pool: sqlx::Pool<DB> = match &self.pool {
            Some(pl) => pl.clone(),
            None => {
                return Err(crate::error::DatabaseError::Sql(
                    crate::error::SqlError::PoolNotDefined("The pool is not defined".to_string()),
                ));
            }
        };

        Ok(Box::new(crate::database::query::SqlQuery::new(pool)))
    }
}
