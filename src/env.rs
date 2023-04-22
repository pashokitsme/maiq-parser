use std::str::FromStr;

use lazy_static::lazy_static;

macro_rules! env_params {
  {$($inner: ty as $tt: ident { $closure: expr } ),*} => {
    $(
      #[derive(Debug, Clone)]
      pub struct $tt($inner);

      impl From<$tt> for $inner {
        fn from(value: $tt) -> $inner {
          value.0
        }
      }

      impl FromStr for $tt {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
          Ok($tt($closure(s).map_err(|_| ())?))
        }
      }
    )*
  };
}

macro_rules! env_default {
  {$($tt: ty => $default: expr),*} => {
    $(
      impl Default for $tt {
        fn default() -> Self { Self($default) }
      }
    )*
  }
}

macro_rules! vars {
  {$($getter: ident ($var_name: ident) -> $ty: tt),*} => {
    lazy_static! {
      $(static ref $var_name: $ty = self::parse_var::<$ty>(stringify!($var_name));)*
    }

    $(pub fn $getter() -> $ty { $var_name.clone() })*

    pub fn init() {
      $(
        self::var(stringify!($var_name))
          .and_then(|x| x.parse::<$ty>().ok())
          .is_none()
          .then(|| {
            #[cfg(not(feature = "cli"))]
            warn!("Value {} of type {} is missing. Fallback to default", stringify!($var_name), stringify!($ty));
            #[cfg(feature = "cli")]
            eprintln!("Value {} of type {} is missing. Fallback to default", stringify!($var_name), stringify!($ty))
          });
      )*
    }
  };
}

pub fn var(var: &'static str) -> Option<String> {
  dotenvy::var(var).ok()
}

pub fn parse_var<T: FromStr + Default>(var: &'static str) -> T {
  self::var(var).and_then(|x| x.parse().ok()).unwrap_or_default()
}

env_params! {
  Vec<String> as Strings { |s: &str| -> Result<Vec<String>, ()> { Ok(s.split(';').map(|s| s.trim().into()).collect::<Vec<String>>()) } }
}

env_default! {
  Strings => vec![]
}

vars! {
  groups(GROUPS) -> Strings
}
