#![allow(unused_variables)]
#![allow(dead_code)]
#![macro_use]

// Macros for functional constructs
#[macro_export]
macro_rules! lambda {
    ($($arg:ident),* => $body:expr) => {
        |$($arg),*| $body
    };
}

#[macro_export]
macro_rules! compose {
    ($f:expr, $g:expr) => {
        |x| $f($g(x))
    };
}

#[macro_export]
macro_rules! map {
    ($func:expr, $list:expr) => {
        $list.iter().map($func).collect::<Vec<_>>()
    };
}

#[macro_export]
macro_rules! curry {
    ($fn:expr) => {
        |x| move |y| $fn(x, y)
    };
}

#[macro_export]
macro_rules! foldl {
    ($func:expr, $init:expr, $list:expr) => {
        $list.iter().fold($init, $func)
    };
}

#[macro_export]
macro_rules! list_comprehension {
    ($expr:expr, $var:ident <- $list:expr, $cond:expr) => {
        $list.iter().filter(|&&$var| $cond).map(|&$var| $expr).collect::<Vec<_>>()
    };
    ($expr:expr, $var:ident <- $list:expr, $cond:expr, $($vars:ident <- $lists:expr, $conds:expr),+) => {
        $list.iter().filter(|&&$var| $cond).flat_map(|&$var| {
            list_comprehension!($expr, $($vars <- $lists, $conds),+)
        }).collect::<Vec<_>>()
    };
}

// Lazy evaluation using Thunks
pub struct Thunk<T> {
    pub computation: Box<dyn Fn() -> T>,
    pub value: Option<T>,
}

impl<T> Thunk<T> {
    pub fn new<F>(f: F) -> Thunk<T>
    where
        F: Fn() -> T + 'static,
    {
        Thunk {
            computation: Box::new(f),
            value: None,
        }
    }

    pub fn evaluate(&mut self) -> &T {
        if self.value.is_none() {
            let computed_value = (self.computation)();
            self.value = Some(computed_value);
        }
        self.value.as_ref().unwrap()
    }
}

// Monadic operations with Maybe type
pub enum Maybe<T> {
    Just(T),
    Nothing,
}

impl<T> Maybe<T> {
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Maybe<U> {
        match self {
            Maybe::Just(value) => Maybe::Just(f(value)),
            Maybe::Nothing => Maybe::Nothing,
        }
    }

    pub fn and_then<U, F: FnOnce(T) -> Maybe<U>>(self, f: F) -> Maybe<U> {
        match self {
            Maybe::Just(value) => f(value),
            Maybe::Nothing => Maybe::Nothing,
        }
    }
    
    pub fn is_nothing(&self) -> bool {
        matches!(self, Maybe::Nothing)
    }
}

// Macros for additional functional constructs
#[macro_export]
macro_rules! filter {
    ($func:expr, $list:expr) => {
        $list.iter().filter(|&x| $func(x)).cloned().collect::<Vec<_>>()
    };
}

#[macro_export]
macro_rules! reduce {
    ($func:expr, $init:expr, $list:expr) => {
        $list.iter().fold($init, |acc, &x| $func(acc, x))
    };
}

#[macro_export]
macro_rules! match_with {
    ($value:expr, { $($pattern:pat => $result:expr),* $(,)? }) => {
        match $value {
            $( $pattern => $result, )*
        }
    };
}

// Introducing an immutable list for pure functional operations
pub enum ImmutableList<T> {
    Cons(T, Box<ImmutableList<T>>),
    Nil,
}

impl<T> ImmutableList<T> {
    pub fn new() -> Self {
        ImmutableList::Nil
    }

    pub fn prepend(self, elem: T) -> Self {
        ImmutableList::Cons(elem, Box::new(self))
    }

    pub fn head(&self) -> Maybe<&T> {
        match self {
            ImmutableList::Cons(head, _) => Maybe::Just(head),
            ImmutableList::Nil => Maybe::Nothing,
        }
    }

    pub fn tail(&self) -> Maybe<&ImmutableList<T>> {
        match self {
            ImmutableList::Cons(_, tail) => Maybe::Just(tail),
            ImmutableList::Nil => Maybe::Nothing,
        }
    }
}

// ZipWith macro - combines two lists with a function
#[macro_export]
macro_rules! zip_with {
    ($func:expr, $list1:expr, $list2:expr) => {
        $list1.iter().zip($list2.iter()).map(|(&x, &y)| $func(x, y)).collect::<Vec<_>>()
    };
}
