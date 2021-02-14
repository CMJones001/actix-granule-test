table! {
    experiment (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
    }
}

table! {
    granule (id) {
        id -> Int4,
        valid -> Bool,
        area -> Nullable<Float4>,
        experiment_id -> Int4,
    }
}

joinable!(granule -> experiment (experiment_id));

allow_tables_to_appear_in_same_query!(
    experiment,
    granule,
);
