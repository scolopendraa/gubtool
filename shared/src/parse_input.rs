use {
    crate::act_array::ActArray,
    std::{any::TypeId, collections::HashMap, sync::LazyLock},
};

pub trait ParseInput: Sized {
    fn parse_input(s: &str) -> Option<Self>;
}

macro_rules! impl_for_int {
    ($($t:ty),*) => {
        $(
            impl ParseInput for $t {
                fn parse_input(s: &str) -> Option<Self> {
                    let s = s.trim();

                    if let Some(hex) = s.strip_prefix("0x") {
                        return <$t>::from_str_radix(hex, 16).ok();
                    }
                    s.parse::<$t>().ok()
                }
            }
        )*
    };
}

macro_rules! impl_for_other {
    ($($t:ty),*) => {
        $(
            impl ParseInput for $t {
                fn parse_input(s: &str) -> Option<Self> {
                    let s = s.trim();

                    s.parse::<$t>().ok()
                }
            }
        )*
    };
}

impl_for_int!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);
impl_for_other!(f32, f64, ActArray);

static REGISTRY: LazyLock<ParseRegistry> = LazyLock::new(ParseRegistry::init);

pub struct ParseRegistry {
    registry: HashMap<TypeId, CanParseFn>,
}

type CanParseFn = fn(&str) -> bool;

impl ParseRegistry {
    pub fn init() -> Self {
        let mut reg = Self {
            registry: HashMap::new(),
        };
        reg.register_all();
        reg
    }

    pub fn can_parse(&self, type_id: TypeId, s: &str) -> bool {
        self.registry.get(&type_id).map(|f| f(s)).unwrap_or(false)
    }

    fn register_all(&mut self) {
        self.register::<u8>();
        self.register::<u16>();
        self.register::<u32>();
        self.register::<u64>();
        self.register::<usize>();
        self.register::<i8>();
        self.register::<i16>();
        self.register::<i32>();
        self.register::<i64>();
        self.register::<isize>();

        self.register::<f32>();
        self.register::<f64>();
        self.register::<ActArray>();
    }

    fn register<T>(&mut self)
    where T: ParseInput + 'static {
        self.registry
            .insert(TypeId::of::<T>(), |s| T::parse_input(s).is_some());
    }
}

pub fn can_input_be_parsed_from_type(type_id: TypeId, s: &str) -> bool {
    REGISTRY.can_parse(type_id, s)
}

pub fn title_case_from_pascal_case(string: &str) -> String {
    let mut formatted = String::new();

    for (i, ch) in string.chars().enumerate() {
        if i > 0 && ch.is_uppercase() {
            formatted.push(' ');
        }
        formatted.push(ch);
    }

    formatted
}

pub fn lower_snake_case_from_pascal_case(string: &str) -> String {
    let mut formatted = String::new();

    for (i, ch) in string.chars().enumerate() {
        if i > 0 && ch.is_uppercase() {
            formatted.push('_');
        }
        formatted.push(ch.to_ascii_lowercase());
    }

    formatted
}
