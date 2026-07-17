//! Module de gestion de l'état global des bases de données de
//! l'application.
//!
//! Ce module expose la structure [`StateDataBase`], qui centralise
//! l'ensemble des pools de connexions SQL (un par alias) et fournit
//! des méthodes d'accès pour les récupérer.

/// État global regroupant l'ensemble des pools de connexions SQL de
/// l'application, indexés par leur alias.
pub struct StateDataBase {
    /// Table associant chaque alias de connexion (`String`) à son
    /// pool de connexions SQL correspondant ([`SqlPool`](crate::database::sql_pool::SqlPool)).
    sql_contections: std::collections::HashMap<String, crate::database::sql_pool::SqlPool>,
}

impl StateDataBase {
    /// Crée une nouvelle instance de [`StateDataBase`] à partir de la
    /// configuration globale de la base de données.
    ///
    /// Cette méthode initialise l'ensemble des pools de connexions SQL
    /// définis dans la configuration, en s'appuyant sur
    /// [`Self::build_sql_connector`].
    ///
    /// # Paramètres
    /// - `config` : la configuration globale (`crate::config::Database`),
    ///   contenant notamment la liste des connecteurs SQL à initialiser.
    ///
    /// # Retour
    /// - `Ok(Self)` si tous les pools ont été créés avec succès.
    /// - `Err(crate::error::DatabaseError)` si l'un des pools échoue à
    ///   se créer.
    pub async fn new(config: crate::config::Database) -> Result<Self, crate::error::DatabaseError> {
        let sql_contections = Self::build_sql_connector(config.sql_connectors).await?;
        Ok(Self { sql_contections })
    }

    /// Construit la table des pools de connexions SQL à partir d'une
    /// liste de configurations de connecteurs.
    ///
    /// Pour chaque configuration fournie, cette méthode crée un
    /// [`SqlPool`](crate::database::sql_pool::SqlPool) et l'insère
    /// dans la table de résultats, en utilisant l'alias de la
    /// configuration (`conf.alias`) comme clé.
    ///
    /// # Paramètres
    /// - `configs` : liste des configurations de connecteurs SQL
    ///   (`Vec<crate::config::SqlConnector>`) à initialiser.
    ///
    /// # Retour
    /// - `Ok(HashMap<String, SqlPool>)` contenant l'ensemble des pools
    ///   créés, indexés par alias.
    /// - `Err(crate::error::DatabaseError)` si la création d'un des
    ///   pools échoue.
    async fn build_sql_connector(
        configs: Vec<crate::config::SqlConnector>,
    ) -> Result<
        std::collections::HashMap<String, crate::database::sql_pool::SqlPool>,
        crate::error::DatabaseError,
    > {
        let mut hasmap: std::collections::HashMap<String, crate::database::sql_pool::SqlPool> =
            std::collections::HashMap::new();

        for conf in configs {
            let name = conf.alias.clone();
            let pool: crate::database::sql_pool::SqlPool =
                crate::database::sql_pool::SqlPool::new(conf).await?;

            hasmap.insert(name, pool);
        }

        Ok(hasmap)
    }
}

impl StateDataBase {
    /// Récupère le pool de connexions SQL correspondant à un alias
    /// donné.
    ///
    /// # Paramètres
    /// - `alias` : le nom (alias) du connecteur SQL recherché.
    ///
    /// # Retour
    /// - `Ok(&SqlPool)` si un pool correspondant à l'alias existe.
    /// - `Err(crate::error::DatabaseError::PoolNotFound)` si aucun
    ///   pool n'est trouvé pour cet alias.
    pub fn get_sql_pool(
        &self,
        alias: String,
    ) -> Result<&crate::database::sql_pool::SqlPool, crate::error::DatabaseError> {
        let pool = match self.sql_contections.get(&alias) {
            Some(pl) => pl,
            None => {
                return Err(crate::error::DatabaseError::PoolNotFound(format!(
                    "The pool '{}' not found!",
                    alias
                )));
            }
        };

        Ok(pool)
    }

    /// Récupère directement la connexion à la base de données
    /// associée à un alias donné.
    ///
    /// Cette méthode est un raccourci qui combine [`Self::get_sql_pool`]
    /// et [`SqlPool::get_connection`](crate::database::sql_pool::SqlPool::get_connection)
    /// afin d'obtenir directement la connexion sans passer
    /// explicitement par le pool.
    ///
    /// # Paramètres
    /// - `alias` : le nom (alias) du connecteur SQL recherché.
    ///
    /// # Retour
    /// - `Ok(&sea_orm::DatabaseConnection)` si le pool correspondant à
    ///   l'alias existe.
    /// - `Err(crate::error::DatabaseError::PoolNotFound)` si aucun
    ///   pool n'est trouvé pour cet alias.
    pub fn get_sql_connection(
        &self,
        alias: String,
    ) -> Result<&sea_orm::DatabaseConnection, crate::error::DatabaseError> {
        let pool: &crate::database::sql_pool::SqlPool = self.get_sql_pool(alias)?;
        let conn: &sea_orm::DatabaseConnection = pool.get_connection();
        Ok(conn)
    }
}
