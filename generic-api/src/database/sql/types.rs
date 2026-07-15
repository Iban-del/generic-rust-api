use crate::database::sql;

/// Représente une valeur typée pouvant être utilisée dans une requête SQL.
///
/// Cette énumération sert de représentation intermédiaire entre les types
/// natifs Rust et leur équivalent formaté pour une requête SQL (via
/// l'implémentation de [`std::fmt::Display`]).
#[derive(Debug, serde::Serialize)]
pub enum SqlType {
    /// Valeur textuelle (sera entourée de guillemets simples lors du formatage).
    Text(String),
    /// Valeur numérique entière non signée.
    UNumber(u64),
    /// Valeur numérique entière signée.
    INumber(i64),
    /// Valeur numérique à virgule flottante.
    Float(f64),
    /// Valeur booléenne.
    Bool(bool),
    /// Liste de valeurs [`SqlType`], formatée comme un tuple SQL `(v1, v2, ...)`.
    List(Vec<SqlType>),
}

/// Convertit une chaîne de caractères empruntée (`&str`) en [`SqlType::Text`].
impl From<&str> for SqlType {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}

/// Convertit une chaîne de caractères possédée (`String`) en [`SqlType::Text`].
impl From<String> for SqlType {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

/// Convertit un booléen en [`SqlType::Bool`].
impl From<bool> for SqlType {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

/// Convertit un `u8` en [`SqlType::UNumber`] (conversion vers `u64`).
impl From<u8> for SqlType {
    fn from(value: u8) -> Self {
        Self::UNumber(value as u64)
    }
}

/// Convertit un `u16` en [`SqlType::UNumber`] (conversion vers `u64`).
impl From<u16> for SqlType {
    fn from(value: u16) -> Self {
        Self::UNumber(value as u64)
    }
}

/// Convertit un `u32` en [`SqlType::UNumber`] (conversion vers `u64`).
impl From<u32> for SqlType {
    fn from(value: u32) -> Self {
        Self::UNumber(value as u64)
    }
}

/// Convertit un `u64` en [`SqlType::UNumber`].
impl From<u64> for SqlType {
    fn from(value: u64) -> Self {
        Self::UNumber(value)
    }
}

/// Convertit un `usize` en [`SqlType::UNumber`] (conversion vers `u64`).
impl From<usize> for SqlType {
    fn from(value: usize) -> Self {
        Self::UNumber(value as u64)
    }
}

/// Convertit un `i8` en [`SqlType::INumber`] (conversion vers `i64`).
impl From<i8> for SqlType {
    fn from(value: i8) -> Self {
        Self::INumber(value as i64)
    }
}

/// Convertit un `i16` en [`SqlType::INumber`] (conversion vers `i64`).
impl From<i16> for SqlType {
    fn from(value: i16) -> Self {
        Self::INumber(value as i64)
    }
}

/// Convertit un `i32` en [`SqlType::INumber`] (conversion vers `i64`).
impl From<i32> for SqlType {
    fn from(value: i32) -> Self {
        Self::INumber(value as i64)
    }
}

/// Convertit un `i64` en [`SqlType::INumber`].
impl From<i64> for SqlType {
    fn from(value: i64) -> Self {
        Self::INumber(value)
    }
}

/// Convertit un `isize` en [`SqlType::INumber`] (conversion vers `i64`).
impl From<isize> for SqlType {
    fn from(value: isize) -> Self {
        Self::INumber(value as i64)
    }
}

/// Convertit un `f32` en [`SqlType::Float`] (conversion vers `f64`).
impl From<f32> for SqlType {
    fn from(value: f32) -> Self {
        Self::Float(value as f64)
    }
}

/// Convertit un `f64` en [`SqlType::Float`].
impl From<f64> for SqlType {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

/// Formate une [`SqlType`] en sa représentation textuelle utilisable
/// directement dans une requête SQL.
///
/// - Les chaînes de caractères sont entourées de guillemets simples (`'...'`).
/// - Les nombres et booléens sont affichés tels quels.
/// - Les listes sont formatées comme un tuple SQL : `(v1, v2, v3)`.
impl std::fmt::Display for SqlType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlType::Text(val) => write!(f, r"'{}'", val),
            SqlType::UNumber(val) => write!(f, "{}", val),
            SqlType::INumber(val) => write!(f, "{}", val),
            SqlType::Float(val) => write!(f, "{}", val),
            SqlType::Bool(val) => write!(f, "{}", val),
            SqlType::List(sql_types) => {
                write!(f, "(")?;

                for (key, tp) in sql_types.iter().enumerate() {
                    if key != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", tp)?;
                }

                write!(f, ")")
            }
        }
    }
}

impl<DB: sqlx::Database> sqlx::Type<DB> for SqlType
where
    String: sqlx::Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <String as sqlx::Type<DB>>::type_info()
    }

    fn compatible(_ty: &DB::TypeInfo) -> bool {
        true
    }
}

impl<'q, DB> sqlx::Encode<'q, DB> for SqlType
where
    DB: sqlx::Database,
    String: sqlx::Encode<'q, DB>,
    u64: sqlx::Encode<'q, DB>,
    i64: sqlx::Encode<'q, DB>,
    f64: sqlx::Encode<'q, DB>,
    bool: sqlx::Encode<'q, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        match self {
            SqlType::Text(val) => val.encode_by_ref(buf),
            SqlType::UNumber(val) => val.encode_by_ref(buf),
            SqlType::INumber(val) => val.encode_by_ref(buf),
            SqlType::Float(val) => val.encode_by_ref(buf),
            SqlType::Bool(val) => val.encode_by_ref(buf),
            SqlType::List(sql_types) => {
                let val = serde_json::to_string(sql_types)?;
                val.encode_by_ref(buf)
            }
        }
    }
}
