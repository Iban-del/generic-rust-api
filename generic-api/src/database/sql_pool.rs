//! Module de gestion du pool de connexions SQL basé sur `sea_orm`.
//!
//! Ce module expose la structure [`SqlPool`], qui encapsule une
//! `sea_orm::DatabaseConnection` et fournit des méthodes utilitaires
//! pour initialiser, vérifier et fermer proprement la connexion à la
//! base de données.

/// Pool de connexions vers la base de données.
///
/// Cette structure encapsule une instance de
/// `sea_orm::DatabaseConnection` afin de centraliser la gestion du
/// cycle de vie de la connexion (création, vérification, fermeture).
#[derive(Debug, Clone)]
pub struct SqlPool {
    /// Connexion active vers la base de données, gérée par `sea_orm`.
    connection: sea_orm::DatabaseConnection,
}

impl SqlPool {
    /// Crée une nouvelle instance de [`SqlPool`] à partir de la
    /// configuration fournie.
    ///
    /// Cette méthode construit les options de connexion, établit la
    /// connexion à la base de données, puis effectue une vérification
    /// (`check`) pour s'assurer que la connexion est fonctionnelle.
    ///
    /// # Paramètres
    /// - `config` : la configuration de connexion SQL
    ///   (`crate::config::SqlConnector`), contenant l'URL de connexion
    ///   ainsi que les paramètres de pool (connexions min/max).
    ///
    /// # Retour
    /// - `Ok(Self)` si la connexion a été établie et validée avec succès.
    /// - `Err(crate::error::DatabaseError)` si la connexion échoue ou
    ///   si la vérification échoue.
    pub async fn new(
        config: crate::config::SqlConnector,
    ) -> Result<Self, crate::error::DatabaseError> {
        let connection = Self::build_database_connection(config).await?;
        Ok(Self { connection })
    }

    /// Ferme proprement la connexion à la base de données.
    ///
    /// Cette méthode consomme `self` (prend possession de l'instance),
    /// ce qui garantit qu'aucune utilisation ultérieure du pool n'est
    /// possible après la fermeture.
    ///
    /// # Retour
    /// - `Ok(())` si la fermeture s'est déroulée sans erreur.
    /// - `Err(crate::error::DatabaseError)` si la fermeture de la
    ///   connexion échoue.
    pub async fn close(self) -> Result<(), crate::error::DatabaseError> {
        self.connection.close().await?;
        Ok(())
    }

    /// Retourne une référence vers la connexion à la base de données
    /// détenue par ce pool.
    ///
    /// Cette méthode permet d'accéder à la connexion sous-jacente sans
    /// en transférer la propriété, afin de l'utiliser (par exemple pour
    /// exécuter des requêtes) sans consommer ni modifier le [`SqlPool`].
    ///
    /// # Retour
    /// - Une référence immuable (`&sea_orm::DatabaseConnection`) vers la
    ///   connexion active.
    pub fn get_connection(&self) -> &sea_orm::DatabaseConnection {
        &self.connection
    }

    /// Construit et initialise une connexion à la base de données à
    /// partir de la configuration fournie.
    ///
    /// Cette méthode :
    /// 1. Crée les options de connexion (`ConnectOptions`) à partir de
    ///    l'URL contenue dans `conf`.
    /// 2. Configure le nombre maximum et minimum de connexions du pool.
    /// 3. Désactive le logging SQL de `sqlx` (`sqlx_logging(false)`).
    /// 4. Établit la connexion effective via `sea_orm::Database::connect`.
    /// 5. Vérifie la validité de la connexion via [`Self::check`].
    ///
    /// # Paramètres
    /// - `conf` : la configuration SQL (`crate::config::SqlConnector`)
    ///   contenant l'URL et les paramètres du pool de connexions.
    ///
    /// # Retour
    /// - `Ok(sea_orm::DatabaseConnection)` si la connexion est établie
    ///   et validée avec succès.
    /// - `Err(crate::error::DatabaseError)` en cas d'échec de connexion
    ///   ou de validation.
    async fn build_database_connection(
        conf: crate::config::SqlConnector,
    ) -> Result<sea_orm::DatabaseConnection, crate::error::DatabaseError> {
        let mut sql_conn_opt = sea_orm::ConnectOptions::new(conf.url);
        sql_conn_opt
            .max_connections(conf.max_connections)
            .min_connections(conf.min_connections)
            .sqlx_logging(false);

        let sql_conn: sea_orm::DatabaseConnection =
            sea_orm::Database::connect(sql_conn_opt).await?;

        Self::check(&sql_conn).await?;

        Ok(sql_conn)
    }

    /// Vérifie l'état fonctionnel d'une connexion à la base de données.
    ///
    /// Cette méthode effectue les vérifications suivantes :
    /// 1. Envoie un ping (`ping`) à la base de données et s'assure que
    ///    celui-ci réussit (`assert!`).
    /// 2. Clone la connexion et la ferme (`close`), simulant ainsi une
    ///    perte de connexion.
    /// 3. Vérifie qu'un nouveau `ping` échoue bien avec une erreur de
    ///    type `sea_orm::DbErr::ConnectionAcquire`, confirmant que la
    ///    connexion a bien été fermée.
    ///
    /// # Paramètres
    /// - `db` : référence vers la connexion à la base de données à
    ///   vérifier.
    ///
    /// # Retour
    /// - `Ok(())` si toutes les assertions sont validées.
    ///
    /// # Panics
    /// - Cette fonction utilise `assert!` : elle paniquera si le
    ///   premier `ping` échoue, ou si le second `ping` (après
    ///   fermeture) ne renvoie pas l'erreur attendue.
    async fn check(db: &sea_orm::DatabaseConnection) -> Result<(), crate::error::DatabaseError> {
        assert!(db.ping().await.is_ok());
        db.clone().close().await;
        assert!(matches!(
            db.ping().await,
            Err(sea_orm::DbErr::ConnectionAcquire(_))
        ));
        Ok(())
    }
}
