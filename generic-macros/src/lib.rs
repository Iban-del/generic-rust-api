use generic_type::routes::{HttpMethod, RouteModuleParams, RouteParams};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Item, ItemFn, ItemMod, ItemStruct, parse_macro_input, parse_quote};

// ---- Macro de gestion de route namespace ----

/// Macros des module pour les routes
#[proc_macro_attribute]
pub fn route_space(attr: TokenStream, item: TokenStream) -> TokenStream {
    let at: RouteModuleParams = parse_macro_input!(attr as RouteModuleParams);
    let base_url: &str = at.base_url;
    let mut module: ItemMod = parse_macro_input!(item as ItemMod);

    let base_urk_const: Item = parse_quote! {
        pub(crate) const BASE_URI: &str = #base_url;
    };

    if let Some((_, items)) = &mut module.content {
        items.insert(0, base_urk_const);
    }

    let expanded = quote! {
        #module
    };
    TokenStream::from(expanded)
}

// ---- Macro de gestion des routes ----

/// Fonction macro pour gerer les route automatiquement
///
/// # Exemple
/// #[route(HttpMethod::GET, "/")]
/// async fn home() -> Json<Value> {
///    Json(json!({ "data": "Hello world!" }))
/// }
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let params: RouteParams = parse_macro_input!(attr as RouteParams);

    let http_method = params.http_method;
    let fn_name = &input_fn.sig.ident;

    // l'uri n'est pas oblicatoire
    let uri = match &params.uri {
        Some(value) => quote! { #value },
        None => {
            let fn_name_str = fn_name.to_string();

            quote! {

                ::const_format::concatcp!(BASE_URI, "/", #fn_name_str)
            }
        }
    };

    // Choix de la fonction axum en fonction de la méthode HTTP
    let method_fn = match http_method {
        HttpMethod::GET => quote! { ::axum::routing::get },
        HttpMethod::POST => quote! { ::axum::routing::post },
        HttpMethod::PUT => quote! { ::axum::routing::put },
        HttpMethod::DELETE => quote! { ::axum::routing::delete },
    };

    let expanded = quote! {
        #input_fn

        ::inventory::submit! {
            ::generic_api::routes::ApiRoute::new(
                #uri,
                || #method_fn(#fn_name)
            )
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn service(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let c_struct: ItemStruct = parse_macro_input!(item as ItemStruct);
    let struct_name = &c_struct.ident;

    let expanded = quote! {
        #c_struct

        ::inventory::submit! {
            ::generic_api::service::ServiceInstance {
                type_service: std::any::TypeId::of::<#struct_name>(),
                builder: |db_state: &generic_api::database::state::StateDataBase| -> Box<dyn std::any::Any + Sync + Send> {
                    Box::new(<#struct_name as ::generic_api::service::StartableService>::build(db_state))
                }
            }
        }
    };

    TokenStream::from(expanded)
}
