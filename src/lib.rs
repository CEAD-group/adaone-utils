// lib.rs

use ada3dp::{
    EventData, FanData, Parameters, PathSegment, Point, Quaternion, ToolPathData, ToolPathGroup,
    Vector3D,
};
use polars::prelude::*;
use prost::Message;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3_polars::PyDataFrame;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufReader, Cursor, Read};
pub mod ada3dp {
    include!(concat!(env!("OUT_DIR"), "/ada3_dp.rs"));
}

use pyo3::{exceptions::PyValueError, PyErr};

/// Core function to decode the file into a Polars DataFrame, without Python error handling
use polars::prelude::*;

fn _ada3dp_to_polars(file_path: &str) -> Result<DataFrame, Box<dyn Error>> {
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
    let mut fans_num = Vec::new(); // For FanData num
    let mut fans_speed = Vec::new(); // For FanData speed
    let mut user_events = Vec::new(); // For EventData num
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

                // External Axes - Vector of f64
                external_axes.push(point.external_axes.clone());

                // Deposition
                deposition.push(point.deposition);

                // Speed
                speed.push(point.speed);

                // Handle Fans - Push the num and speed for each FanData
                let fan_data_num = point.fans.iter().map(|fan| fan.num).collect::<Vec<_>>();
                let fan_data_speed = point.fans.iter().map(|fan| fan.speed).collect::<Vec<_>>();
                fans_num.push(fan_data_num);
                fans_speed.push(fan_data_speed);

                // Handle User Events - Push the num for each EventData
                let user_event_data = point
                    .user_events
                    .iter()
                    .map(|event| event.num)
                    .collect::<Vec<_>>();
                user_events.push(user_event_data);

                // Other segment data
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
        // New Columns for fans and user events (List of integers)
        Series::new(
            "fans.num".into(),
            ListChunked::from_iter(fans_num.into_iter().map(|v| Series::new("".into(), v))),
        )
        .into(),
        Series::new(
            "fans.speed".into(),
            ListChunked::from_iter(fans_speed.into_iter().map(|v| Series::new("".into(), v))),
        )
        .into(),
        Series::new(
            "userEvents.num".into(),
            ListChunked::from_iter(user_events.into_iter().map(|v| Series::new("".into(), v))),
        )
        .into(),
        // External Axes - List of floats
        Series::new(
            "externalAxes".into(),
            ListChunked::from_iter(external_axes.into_iter().map(|v| Series::new("".into(), v))),
        )
        .into(),
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

fn _polars_to_ada3dp(df: DataFrame) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut tool_path_data = ToolPathData {
        tool_path_groups: Vec::new(),
        parameters: vec![Parameters {
            deposition_width: 0.0,
            layer_height: 0.0,
            path_planning_strategy: 0,
            posi_axis1_val: 0.0,
            posi_axis2_val: 0.0,
            posi_axis1_dynamic: false,
            posi_axis2_dynamic: false,
        }],
    };

    // Ensure the "layerIndex" column exists
    if !df.get_column_names().iter().any(|&col| col == "layerIndex") {
        return Err(Box::new(PolarsError::ColumnNotFound(
            "layerIndex column not found".into(),
        )));
    }

    let grouped_layers = df.partition_by_stable(["layerIndex"], true)?;
    for layer_df in grouped_layers {
        let layer_index = layer_df.column("layerIndex")?.i32()?.get(0).unwrap_or(0);

        let mut group = ToolPathGroup {
            layer_index,
            path_segments: Vec::new(),
        };

        let grouped_segments = layer_df.partition_by_stable(["segmentID"], true)?;
        for segment_df in grouped_segments {
            let segment_id = segment_df.column("segmentID")?.i32()?.get(0).unwrap_or(0);

            let mut path_segment = PathSegment {
                points: Vec::new(),
                process_on: false,
                r#type: segment_df.column("type")?.i32()?.get(0).unwrap_or(0),
                process_on_delay: segment_df
                    .column("processOnDelay")?
                    .f32()?
                    .get(0)
                    .unwrap_or(0.0),
                process_off_delay: segment_df
                    .column("processOffDelay")?
                    .f32()?
                    .get(0)
                    .unwrap_or(0.0),
                start_delay: segment_df
                    .column("startDelay")?
                    .f32()?
                    .get(0)
                    .unwrap_or(0.0),
                end_delay: 0.0,
                speed_tcp: segment_df.column("speedTCP")?.i32()?.get(0).unwrap_or(0),
                equipment_id: segment_df.column("equipmentID")?.i32()?.get(0).unwrap_or(0),
                tool_id: segment_df.column("toolID")?.i32()?.get(0).unwrap_or(0),
                material_id: segment_df.column("materialID")?.i32()?.get(0).unwrap_or(0),
            };

            let columns = [
                "position.x",
                "position.y",
                "position.z",
                "direction.x",
                "direction.y",
                "direction.z",
                "orientation.x",
                "orientation.y",
                "orientation.z",
                "orientation.w",
                "deposition",
                "speed",
            ];

            let mut iters = columns
                .iter()
                .map(|&col| segment_df.column(col)?.f64().map(|s| s.into_iter()))
                .collect::<Result<Vec<_>, PolarsError>>()?;

            for i in 0..segment_df.height() {
                let point = Point {
                    position: Some(Vector3D {
                        x: iters[0].next().flatten().unwrap_or(f64::NAN),
                        y: iters[1].next().flatten().unwrap_or(f64::NAN),
                        z: iters[2].next().flatten().unwrap_or(f64::NAN),
                    }),
                    direction: Some(Vector3D {
                        x: iters[3].next().flatten().unwrap_or(f64::NAN),
                        y: iters[4].next().flatten().unwrap_or(f64::NAN),
                        z: iters[5].next().flatten().unwrap_or(f64::NAN),
                    }),
                    orientation: Some(Quaternion {
                        x: iters[6].next().flatten().unwrap_or(f64::NAN),
                        y: iters[7].next().flatten().unwrap_or(f64::NAN),
                        z: iters[8].next().flatten().unwrap_or(f64::NAN),
                        w: iters[9].next().flatten().unwrap_or(f64::NAN),
                    }),
                    deposition: iters[10].next().flatten().unwrap_or(f64::NAN),
                    speed: iters[11].next().flatten().unwrap_or(f64::NAN),
                    external_axes: vec![],
                    fans: vec![],
                    user_events: vec![],
                };

                path_segment.points.push(point);
            }

            group.path_segments.push(path_segment);
        }

        tool_path_data.tool_path_groups.push(group);
    }

    let mut buf = Vec::new();
    tool_path_data.encode(&mut buf)?;
    Ok(buf)
}

#[pyfunction]
fn polars_to_ada3dp(df: PyDataFrame, file_path: &str) -> PyResult<()> {
    let df: DataFrame = df.into();
    let serialized_data = _polars_to_ada3dp(df).map_err(|e| {
        PyErr::new::<PyValueError, _>(format!(
            "Error converting Polars DataFrame to ToolPathData: {}",
            e
        ))
    })?;

    let mut file = File::create(file_path).map_err(|e| {
        PyErr::new::<PyValueError, _>(format!("Error creating file {}: {}", file_path, e))
    })?;
    file.write_all(&serialized_data).map_err(|e| {
        PyErr::new::<PyValueError, _>(format!("Error writing to file {}: {}", file_path, e))
    })?;

    Ok(())
}

#[pymodule]
fn py_adaone(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ada3dp_to_polars, m)?)?;
    m.add_function(wrap_pyfunction!(polars_to_ada3dp, m)?)?;
    Ok(())
}
