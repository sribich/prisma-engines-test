use enumflags2::*;

macro_rules! tags {
    ($($name:ident = $pattern:expr,)*) => {
        /// Test-relevant connector tags.
        #[bitflags]
        #[derive(Copy, Clone, Debug, PartialEq)]
        #[repr(u32)]
        pub enum Tags {
            $($name = $pattern,)*
        }

        const ALL_TAG_NAMES: &[(&str, Tags)] = &[
            $(
                (stringify!($name), Tags::$name),
            )*
        ];
    }
}

tags![
    LowerCasesTableNames = 1 << 0,
    Mysql = 1 << 1,
    Mariadb = 1 << 2,
    Postgres = 1 << 3,
    Sqlite = 1 << 4,
    Mysql8 = 1 << 5,
    Mysql56 = 1 << 6,
    Mysql57 = 1 << 7,
    Postgres12 = 1 << 8,
    Vitess = 1 << 9,
    Postgres14 = 1 << 10,
    Postgres9 = 1 << 11,
    Postgres15 = 1 << 12,
    Postgres11 = 1 << 13,
    Postgres13 = 1 << 14,
    Postgres16 = 1 << 15,
];

pub fn tags_from_comma_separated_list(input: &str) -> BitFlags<Tags> {
    let mut tags = Default::default();

    for s in input.split(',').map(|s| s.trim()) {
        match ALL_TAG_NAMES.iter().find(|(name, _t)| name.eq_ignore_ascii_case(s)) {
            Some((_, tag)) => tags |= *tag,
            None => panic!("unknown tag: {s}"),
        }
    }

    tags
}
