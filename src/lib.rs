// lib.rs

use ada3dp::{Parameters, PathSegment, Point, Quaternion, ToolPathData, ToolPathGroup, Vector3D};
use polars::prelude::*;
use prost::Message;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3_polars::PyDataFrame;
use std::fs::File;
use std::io::{BufReader, Cursor, Read};

pub mod ada3dp {
    include!(concat!(env!("OUT_DIR"), "/ada3_dp.rs"));
}

use pyo3::{exceptions::PyValueError, PyErr};

/// Core function to decode the file into a Polars DataFrame, without Python error handling
fn _ada3dp_to_polars(file_path: &str) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let tool_path_data = ToolPathData::decode(&mut Cursor::new(&buf))?;

    // Preallocate vectors for each column
    let mut pos_x = Vec::new();
    let mut pos_y = Vec::new();
    let mut pos_z = Vec::new();
    let mut dir_x = Vec::new();
    let mut dir_y = Vec::new();
    let mut dir_z = Vec::new();
    let mut ori_x = Vec::new();
    let mut ori_y = Vec::new();
    let mut ori_z = Vec::new();
    let mut ori_w = Vec::new();
    let mut external_axes = Vec::new();
    let mut deposition = Vec::new();
    let mut speed = Vec::new();
    let mut fans = Vec::new();
    let mut user_events = Vec::new();
    let mut speed_tcp = Vec::new();
    let mut segment_type = Vec::new();
    let mut layer_index = Vec::new();
    let mut process_on_delay = Vec::new();
    let mut process_off_delay = Vec::new();
    let mut start_delay = Vec::new();
    let mut equipment_id = Vec::new();
    let mut tool_id = Vec::new();
    let mut material_id = Vec::new();
    let mut segment_id = Vec::new();

    let mut segment_counter = 0;

    for group in tool_path_data.tool_path_groups.iter() {
        for segment in &group.path_segments {
            for point in &segment.points {
                if let Some(position) = &point.position {
                    pos_x.push(position.x);
                    pos_y.push(position.y);
                    pos_z.push(position.z);
                } else {
                    pos_x.push(f64::NAN);
                    pos_y.push(f64::NAN);
                    pos_z.push(f64::NAN);
                }

                if let Some(direction) = &point.direction {
                    dir_x.push(direction.x);
                    dir_y.push(direction.y);
                    dir_z.push(direction.z);
                } else {
                    dir_x.push(f64::NAN);
                    dir_y.push(f64::NAN);
                    dir_z.push(f64::NAN);
                }

                if let Some(orientation) = &point.orientation {
                    ori_x.push(orientation.x);
                    ori_y.push(orientation.y);
                    ori_z.push(orientation.z);
                    ori_w.push(orientation.w);
                } else {
                    ori_x.push(f64::NAN);
                    ori_y.push(f64::NAN);
                    ori_z.push(f64::NAN);
                    ori_w.push(f64::NAN);
                }

                external_axes.push(point.external_axes.clone());
                deposition.push(point.deposition);
                speed.push(point.speed);
                fans.push(point.fans.clone());
                user_events.push(point.user_events.clone());

                speed_tcp.push(segment.speed_tcp);
                segment_type.push(segment.r#type);
                layer_index.push(group.layer_index);
                process_on_delay.push(segment.process_on_delay);
                process_off_delay.push(segment.process_off_delay);
                start_delay.push(segment.start_delay);
                equipment_id.push(segment.equipment_id);
                tool_id.push(segment.tool_id);
                material_id.push(segment.material_id);
                segment_id.push(segment_counter);
            }
            segment_counter += 1;
        }
    }

    // Convert collected data into Polars DataFrame
    let df = DataFrame::new(vec![
        Series::new("position.x".into(), pos_x).into(),
        Series::new("position.y".into(), pos_y).into(),
        Series::new("position.z".into(), pos_z).into(),
        Series::new("direction.x".into(), dir_x).into(),
        Series::new("direction.y".into(), dir_y).into(),
        Series::new("direction.z".into(), dir_z).into(),
        Series::new("orientation.x".into(), ori_x).into(),
        Series::new("orientation.y".into(), ori_y).into(),
        Series::new("orientation.z".into(), ori_z).into(),
        Series::new("orientation.w".into(), ori_w).into(),
        Series::new("deposition".into(), deposition).into(),
        Series::new("speed".into(), speed).into(),
        Series::new("speedTCP".into(), speed_tcp).into(),
        Series::new("type".into(), segment_type).into(),
        Series::new("layerIndex".into(), layer_index).into(),
        Series::new("processOnDelay".into(), process_on_delay).into(),
        Series::new("processOffDelay".into(), process_off_delay).into(),
        Series::new("startDelay".into(), start_delay).into(),
        Series::new("equipmentID".into(), equipment_id).into(),
        Series::new("toolID".into(), tool_id).into(),
        Series::new("materialID".into(), material_id).into(),
        Series::new("segmentID".into(), segment_id).into(),
    ])?;

    Ok(df)
}

/// Converts the result of _ada3dp_to_polars into a Python DataFrame, mapping any errors to PyValueError
#[pyfunction(signature = (file_path))]
fn ada3dp_to_polars(file_path: &str) -> PyResult<PyDataFrame> {
    _ada3dp_to_polars(file_path)
        .map_err(|e| {
            PyErr::new::<PyValueError, _>(format!("Error converting to Polars DataFrame: {}", e))
        })
        .map(|df| PyDataFrame(df))
}

#[pymodule]
fn py_adaone(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ada3dp_to_polars, m)?)?;
    Ok(())
}
