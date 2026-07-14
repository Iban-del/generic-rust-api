use generic_tool::parse;
use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::{Ident, LitStr, Token, parse::Parse};

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl Parse for HttpMethod {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Ident) && input.peek2(Token![::]) {
            input.parse::<Ident>()?;
            input.parse::<Token![::]>()?;
        }

        let iden: Ident = input.parse()?;
        let method: HttpMethod = match iden.to_string().as_str() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            _ => return Err(syn::Error::new_spanned(iden, "unsupported HTTP method")),
        };

        Ok(method)
    }
}

impl ToTokens for HttpMethod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(proc_macro2::Ident::new(
            "HttpMethod",
            proc_macro2::Span::call_site(),
        ));
        tokens.append(proc_macro2::Punct::new(':', proc_macro2::Spacing::Joint));
        tokens.append(proc_macro2::Punct::new(':', proc_macro2::Spacing::Alone));
        match *self {
            HttpMethod::DELETE => tokens.append(proc_macro2::Ident::new(
                "DELETE",
                proc_macro2::Span::call_site(),
            )),
            HttpMethod::GET => tokens.append(proc_macro2::Ident::new(
                "GET",
                proc_macro2::Span::call_site(),
            )),
            HttpMethod::POST => tokens.append(proc_macro2::Ident::new(
                "POST",
                proc_macro2::Span::call_site(),
            )),
            HttpMethod::PUT => tokens.append(proc_macro2::Ident::new(
                "PUT",
                proc_macro2::Span::call_site(),
            )),
        }
    }
}

pub struct RouteParams {
    pub http_method: HttpMethod,
    pub uri: Option<String>,
}

impl syn::parse::Parse for RouteParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let http_method: HttpMethod = input.parse()?;
        let uri = if input.peek(syn::Token![,]) {
            input.parse::<syn::Token![,]>()?;
            let lit: syn::LitStr = input.parse()?;
            Some(lit.value())
        } else {
            None
        };

        Ok(RouteParams { http_method, uri })
    }
}

pub struct RouteModuleParams {
    pub base_url: &'static str,
}

impl Parse for RouteModuleParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let base_url: &'static str =
            Box::leak(parse::<LitStr, Token![,]>(input)?.value().into_boxed_str());

        Ok(RouteModuleParams { base_url })
    }
}
