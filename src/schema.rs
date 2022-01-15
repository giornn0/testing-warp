table! {
    contactos (id) {
        id -> Int4,
        telefono -> Varchar,
        email -> Varchar,
        detalles -> Varchar,
        jardin_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    jardines (id) {
        id -> Int4,
        nombre -> Varchar,
        domicilio -> Varchar,
        telefono -> Varchar,
        email -> Varchar,
        estado -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(contactos -> jardines (jardin_id));

allow_tables_to_appear_in_same_query!(
    contactos,
    jardines,
);
