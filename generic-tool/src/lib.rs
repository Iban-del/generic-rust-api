use local_ip_address::linux::local_ip;
use serde::de::DeserializeOwned;
use std::{
    fs::{self},
    io,
};
use syn::parse::Parse;

// ---- Gestion fichier ----

/// Lit un fichier json
///
/// # Exemple
/// let contenue : Vec<MaStructure> = read_json::<MaStructure>("monFichier.json")?;
///
/// # Error
/// Erreur d'ouverture, de lecture ou de désérialisation des données
///
/// # Return
/// Vecteur de la structure
pub fn read_json<T: DeserializeOwned>(path: String) -> Result<T, io::Error> {
    let content = fs::read_to_string(path)?;
    let value: T = serde_json::from_str(&content)?;

    Ok(value)
}

/// Permet de passer en sautant un délimiteur
///
/// # Exemple
/// let value = parse::<syn::Ident, syn::Token![,]>(input)?;
///
/// # Type
/// P : Parse (Type de donnée renvoyée)
/// T : Parse (Token à sauter)
///
/// # Return
/// Valeur parsée
///
pub fn parse<P: Parse, T: Parse>(input: syn::parse::ParseStream) -> Result<P, syn::Error> {
    let value: P = input.parse()?;
    let _ = input.parse::<T>();
    Ok(value)
}

/// Renvoi l'ip de l'hôte
pub fn get_ip() -> Result<String, local_ip_address::Error> {
    let ip = local_ip()?;
    Ok(ip.to_string())
}
