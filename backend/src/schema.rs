// @generated automatically by Diesel CLI.

diesel::table! {
    dyno_info (id) {
        id -> Integer,
        motor_type -> Integer,
        name -> Nullable<Text>,
        cc -> Nullable<Integer>,
        cylinder -> Nullable<Integer>,
        stroke -> Nullable<Integer>,
        diameter_roller -> Nullable<Float>,
        diameter_roller_beban -> Nullable<Float>,
        diameter_gear_encoder -> Nullable<Float>,
        diameter_gear_beban -> Nullable<Float>,
        jarak_gear -> Nullable<Float>,
        berat_beban -> Nullable<Float>,
        gaya_beban -> Nullable<Float>,
        keliling_roller -> Nullable<Float>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    dynos (id) {
        id -> Integer,
        user_id -> Integer,
        info_id -> Nullable<Integer>,
        uuid -> Text,
        data_checksum -> Text,
        verified -> Nullable<Bool>,
        start -> Timestamp,
        stop -> Timestamp,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        uuid -> Text,
        nim -> Text,
        name -> Text,
        password -> Text,
        role -> Text,
        email -> Nullable<Text>,
        photo -> Nullable<Text>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(dyno_info, dynos, users,);
