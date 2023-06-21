use crate::schema::dyno_info;
use dyno_core::chrono::NaiveDateTime;
use dyno_core::model::dynotests::MotorTy;
use dyno_core::{serde, DynoConfig, ElectricMotor, InfoMotor, Numeric};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Selectable,
)]
#[serde(crate = "serde")]
#[diesel(table_name = dyno_info)]
pub struct DynoInfo {
    pub id: i64,
    pub motor_type: i16,
    pub name: Option<String>,
    pub cc: Option<i16>,
    pub cylinder: Option<i16>,
    pub stroke: Option<i16>,
    pub diameter_roller: Option<f32>,
    pub diameter_roller_beban: Option<f32>,
    pub diameter_gear_encoder: Option<f32>,
    pub diameter_gear_beban: Option<f32>,
    pub jarak_gear: Option<f32>,
    pub berat_beban: Option<f32>,
    pub gaya_beban: Option<f32>,
    pub keliling_roller: Option<f32>,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Default, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[serde(crate = "serde")]
#[diesel(table_name = dyno_info)]
pub struct NewDynoInfo {
    pub motor_type: i16,
    pub name: Option<String>,
    pub cc: Option<i16>,
    pub cylinder: Option<i16>,
    pub stroke: Option<i16>,
    pub diameter_roller: Option<f32>,
    pub diameter_roller_beban: Option<f32>,
    pub diameter_gear_encoder: Option<f32>,
    pub diameter_gear_beban: Option<f32>,
    pub jarak_gear: Option<f32>,
    pub berat_beban: Option<f32>,
    pub gaya_beban: Option<f32>,
    pub keliling_roller: Option<f32>,
}

impl NewDynoInfo {
    pub fn from_dyno_config(
        DynoConfig {
            motor_type,
            diameter_roller,
            diameter_roller_beban,
            diameter_gear_encoder,
            diameter_gear_beban,
            jarak_gear,
            berat_beban,
            gaya_beban,
            keliling_roller,
            ..
        }: DynoConfig,
    ) -> Self {
        match motor_type {
            dyno_core::MotorType::Electric(ElectricMotor { name }) => Self {
                motor_type: MotorTy::Electric as i16,
                name: Some(name),
                diameter_roller: Some(diameter_roller.to_f32()),
                diameter_roller_beban: Some(diameter_roller_beban.to_f32()),
                diameter_gear_encoder: Some(diameter_gear_encoder.to_f32()),
                diameter_gear_beban: Some(diameter_gear_beban.to_f32()),
                jarak_gear: Some(jarak_gear.to_f32()),
                berat_beban: Some(berat_beban.to_f32()),
                gaya_beban: Some(gaya_beban.to_f32()),
                keliling_roller: Some(keliling_roller.to_f32()),
                ..Default::default()
            },
            dyno_core::MotorType::Engine(InfoMotor {
                name,
                cc,
                cylinder,
                stroke,
                ..
            }) => Self {
                motor_type: MotorTy::Electric as _,
                name: Some(name),
                diameter_roller: Some(diameter_roller.to_f32()),
                diameter_roller_beban: Some(diameter_roller_beban.to_f32()),
                diameter_gear_encoder: Some(diameter_gear_encoder.to_f32()),
                diameter_gear_beban: Some(diameter_gear_beban.to_f32()),
                jarak_gear: Some(jarak_gear.to_f32()),
                berat_beban: Some(berat_beban.to_f32()),
                gaya_beban: Some(gaya_beban.to_f32()),
                keliling_roller: Some(keliling_roller.to_f32()),
                cc: Some(cc as _),
                cylinder: Some(cylinder as _),
                stroke: Some(stroke as _),
            },
        }
    }
}

impl From<DynoConfig> for NewDynoInfo {
    fn from(value: DynoConfig) -> Self {
        Self::from_dyno_config(value)
    }
}
